extern crate byteorder;
extern crate simple_error;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;

use self::byteorder::{ByteOrder, LittleEndian};
use path::PathManager;

// What's the .GRP file format?
//
// The ".grp" file format is just a collection of a lot of files stored into 1 big
// one. I tried to make the format as simple as possible: The first 12 bytes
// contains my name, "KenSilverman". The next 4 bytes is the number of files that
// were compacted into the group file. Then for each file, there is a 16 byte
// structure, where the first 12 bytes are the filename, and the last 4 bytes are
// the file's size. The rest of the group file is just the raw data packed one
// after the other in the same order as the list of files.

/// Implementation of a group file "cache", into which the contents of several
/// group files can be loaded. This is somewhat similar to the way that
/// Silverman's original code goes about loading game data.
#[derive(Debug)]
pub struct GroupManager {
    path_manager: PathManager,
    files: HashMap<String, Vec<u8>>,
}

impl GroupManager {
    pub fn new(path_manager: PathManager) -> GroupManager {
        GroupManager { path_manager, files: HashMap::new() }
    }

    /// Loads the contents of an in-memory group file into the cache.
    ///
    /// # Errors
    ///
    /// The operation will fail on any sort of parsing error - such as an
    /// invalid header, or sizes that would cause an invalid read.
    pub fn load_data(&mut self, data: &[u8]) -> Result<(), Box<Error>> {
        let len = data.len();

        if len < 16 {
            bail!("'data' is too small to contain the GRP header.");
        }

        let header = String::from_utf8(data[..12].to_vec())?;

        if header.as_str() != "KenSilverman" {
            bail!("Invalid GRP header.");
        }

        let file_count = LittleEndian::read_u32(&data[12..16]) as usize;

        // 16 bytes for the header, and 16 bytes for each table entry. The raw
        // data will follow.
        let data_start = 16 * (file_count + 1) as usize;

        if data_start >= len {
            bail!("Invalid number of files.");
        }

        let mut data_off = data_start;

        for i in 0..file_count {
            // Similar to how 'data_start' was calculated - 16 bytes for the
            // header, and 16 bytes for each table entry.
            let table_off = 16 * (i + 1);

            let name = &data[table_off..table_off+12];
            let name = String::from_utf8(name.to_vec())?;
            let name = if let Some(j) = name.find('\x00') {
                 String::from(&name[..j])
            } else {
                name
            };

            let size = &data[table_off+12..table_off+16];
            let size = LittleEndian::read_u32(size) as usize;

            if data_off + size > len {
                bail!("`data_off >= len` - Table was likely corrupted.");
            }

            let data = data[data_off..data_off+size].to_vec();
            data_off += size;

            self.files.insert(name, data);
        }

        Ok(())
    }

    /// Queries the associated PathManager for the location of a file with the
    /// given name, and if found, loads its contents.
    ///
    /// # Errors
    ///
    /// A return value of 'Err' indicates that the given path did not exist.
    pub fn load_file(&mut self, name: &str) -> Result<(), Box<Error>> {
        if let Some(path) = self.path_manager.find(name) {
            let mut file = File::open(path)?;
            let mut bytes: Vec<u8> = Vec::new();

            file.read_to_end(&mut bytes)?;
            self.load_data(&bytes)?;

            return Ok(());
        }

        bail!("File not found in any search paths.")
    }

    /// Obtains binary data associated with the given filename from the cache.
    pub fn get(&self, filename: &str) -> Option<&[u8]> {
        Some(&self.files.get(filename)?)
    }
}

#[cfg(test)]
mod grp_tests {
    use super::*;

    #[test]
    fn test_load_slice() {
        // Binary blob containing a GRP test vector, made by me. Contains the
        // "KenSilverman" header, and a table consisting of 3 files:
        //
        // - 'TESTFILEA': A single byte, 0x01.
        // - 'TESTFILEB': 0x02, repeated twice.
        // - 'TESTFILEC': 0x03, repeated three times.
        //
        // The sizes listed in the table accurately represent this.
        let data = vec![
            b'K', b'e', b'n', b'S', b'i', b'l', b'v', b'e',
            b'r', b'm', b'a', b'n', 0x03, 0x00, 0x00, 0x00,
            b'T', b'E', b'S', b'T', b'F', b'I', b'L', b'E',
            b'A', 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
            b'T', b'E', b'S', b'T', b'F', b'I', b'L', b'E',
            b'B', 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
            b'T', b'E', b'S', b'T', b'F', b'I', b'L', b'E',
            b'C', 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,
            0x01, 0x02, 0x02, 0x03, 0x03, 0x03,
        ];

        let path_manager = PathManager::new();
        let mut group_manager = GroupManager::new(path_manager);

        match group_manager.load_data(&data) {
            Err(e) => panic!("{}", e),
            Ok(_) => (),
        }

        let data = match group_manager.get("TESTFILEA") {
            Some(data) => data,
            None => panic!("TESTFILEA wasn't found in the archive"),
        };

        assert_eq!(data.len(), 1);
        assert_eq!(data[0], 0x01);

        let data = match group_manager.get("TESTFILEB") {
            Some(data) => data,
            None => panic!("TESTFILEB wasn't found in the archive"),
        };

        assert_eq!(data.len(), 2);
        assert_eq!(data[0], 0x02);
        assert_eq!(data[1], 0x02);

        let data = match group_manager.get("TESTFILEC") {
            Some(data) => data,
            None => panic!("TESTFILEC wasn't found in the archive"),
        };

        assert_eq!(data.len(), 3);
        assert_eq!(data[0], 0x03);
        assert_eq!(data[1], 0x03);
        assert_eq!(data[2], 0x03);
    }

    #[test]
    fn test_incomplete_header() {
        // Binary blob similar to the GRP test vector above, but with a header
        // that would be too small to be valid.
        let data = vec![
            b'J', b'a', b'k', b'o', b'b',
        ];

        let path_manager = PathManager::new();
        let mut group_manager = GroupManager::new(path_manager);

        if let Ok(_) = group_manager.load_data(&data) {
            panic!("Accepted invalid header.");
        }
    }

    #[test]
    fn test_invalid_header() {
        // Binary blob similar to the GRP test vector above, but with an invalid
        // "magic" header.
        let data = vec![
            b'J', b'a', b'k', b'o', b'b', b'L', b'K', b'r',
            b'e', b'u', b'z', b'e', 0x01, 0x00, 0x00, 0x00,
            b'T', b'E', b'S', b'T', b'F', b'I', b'L', b'E',
            b'A', 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
            0x01,
        ];

        let path_manager = PathManager::new();
        let mut group_manager = GroupManager::new(path_manager);

        if let Ok(_) = group_manager.load_data(&data) {
            panic!("Accepted invalid header.");
        }
    }

    #[test]
    fn test_invalid_file_count() {
        // Binary blob similar to the GRP test vector above, but with a header
        // indicating that there are more files than could possibly be contained
        // in the table.
        let data = vec![
            b'K', b'e', b'n', b'S', b'i', b'l', b'v', b'e',
            b'r', b'm', b'a', b'n', 0x69, 0x00, 0x00, 0x00,
            b'T', b'E', b'S', b'T', b'F', b'I', b'L', b'E',
            b'A', 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
            0x01,
        ];

        let path_manager = PathManager::new();
        let mut group_manager = GroupManager::new(path_manager);

        if let Ok(_) = group_manager.load_data(&data) {
            panic!("Accepted invalid header.");
        }
    }

    #[test]
    fn test_not_enough_data() {
        // Binary blob similar to the GRP test vector above, but with a file
        // entry larger than the data following the table.
        let data = vec![
            b'K', b'e', b'n', b'S', b'i', b'l', b'v', b'e',
            b'r', b'm', b'a', b'n', 0x69, 0x00, 0x00, 0x00,
            b'T', b'E', b'S', b'T', b'F', b'I', b'L', b'E',
            b'A', 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
            0x01,
        ];

        let path_manager = PathManager::new();
        let mut group_manager = GroupManager::new(path_manager);

        if let Ok(_) = group_manager.load_data(&data) {
            panic!("Accepted invalid header.");
        }
    }
}

// What's the PALETTE.DAT format?
//
// char palette[768], palookup[numpalookups][256], transluc[256][256];
// short numpalookups;
//
// fil = open("PALETTE.DAT",...);
// read(fil,palette,768);
// read(fil,&numpalookups,2);
// read(fil,palookup,numpalookups*256);
// read(fil,transluc,65536);
// close(fil);
//
// PALETTE: This 768 byte array is exactly the palette you want. The format is:
// Red0, Green0, Blue0, Red1, Green1, Blue1, ..., Blue255
// The colors are based on the VGA 262,144 color palette. The values range from
// 0-63, so if you want to convert it to a windows palette you will have to
// multiply each byte by 4.
//
// NUMPALOOKUPS: The number of shading tables used. Usually this number is 32,
// but 16 or 64 have also been used. Each of the 256 colors of the VGA palette
// can take on any of "numpalookups" number of shades.
//
// PALOOKUP: The shading table. If numpalookups = 32, then this table is:
// (32 shades) * (256 colors) = 8192 bytes (8K). The shade tables are often made
// to go from normal brightness (shade #0) down to pitch black (shade #31) So
// the first 256 bytes of the table would be for shade #0, etc...
//
// TRANSLUC: 64K translucent lookup table. Given any 2 colors of the palette,
// this lookup table gives the best match of the 2 colors when mixed together.
//
// Here's a funny story: I noticed that Duke3D's PALETTE.DAT file is 8K longer
// than it should be. Any PALETTE.DAT file with 32 shades and translucent table
// should be 74,498 bytes. Duke3D's palette is 82,690 bytes, but it only has 32
// shades! The reason is that at one time, Duke3D had 64 shades in their
// "palookup" table. Then when we noticed that this extra memory overhead slowed
// down the frame rate of the game noticably, it was converted back to 32
// shades. The problem is that my palette conversion program never truncated off
// the end of the file. So the last 8K of Duke3D's PALETTE.DAT is the last 8K of
// a translucent table that was based on an older version of their palette.

/// Parser for PALETTE.DAT, the file specifying the color format.
pub struct Palette {
    colors: Vec<u8>,
}

impl Palette {
    /// Parse the contents of a PALETTE.DAT
    pub fn new(data: &[u8]) -> Result<Palette, Box<Error>> {
        let len = data.len();

        // FIXME: This only takes into account the actual palette. PALETTE.DAT
        // should also contain some lookup tables.
        if len < 770 {
            bail!("Too small to contain palette.");
        }

        let colors = data[0..768].to_vec();

        // FIXME: Not loading the lookup table yet because.. well, I don't know
        // if we really need it yet? I suppose we'll need to get the values for
        // TRANSLUC, but we're not on DOS anymore and I think a lookup table
        // would be overkill. My plan is to convert ART files into bitmaps ahead
        // of time, anyway.

        // let _lookup_count = LittleEndian::read_u16(size) as usize;

        Ok(Palette { colors })
    }
}

#[cfg(test)]
mod palette_tests {
    use super::*;

    #[test]
    fn test_load_slice() {
        // Considering the size of PALETTE.DAT, it would be absurd embed as a
        // blob in this file. We'll just generate dummy data. I'm leaving the
        // number of 'pa' lookups as 0 intentionally.
        let data = [0; 0x10301];

        if let Err(e) = Palette::new(&data) {
            panic!("Valid PALETTE errored out with '{}'", e);
        }
    }

    #[test]
    fn test_not_enough_data() {
        let data = [0; 1];

        if let Ok(_) = Palette::new(&data) {
            panic!("Accepted incomplete header.");
        }
    }
}
