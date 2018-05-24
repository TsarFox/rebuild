extern crate byteorder;
extern crate simple_error;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use self::byteorder::{ByteOrder, LittleEndian};


// FIXME: This doesn't follow any non-*NIX conventions.

/// Means of locating a group file that may be located in any number of file
/// system locations. Begins by searching the current working directory, and
/// then moves onto standard *NIX home directory locations, such as
/// '~/.rebuild'.

fn find_file(filename: &str) -> Option<String> {
    // Initial base paths that don't need to be expanded.
    let directories = vec!["./"];

    let mut directories: Vec<String> = directories.iter()
        .map(|s| String::from(*s))
        .collect();

    match env::home_dir() {
        Some(path) => {
            match path.to_str() {
                Some(path) => {
                    let mut path = String::from(path);
                    path.push_str("/.rebuild/");
                    directories.push(path);
                }
                None => (),
            }
        }
        None => (),
    }

    for root in directories.iter() {
        let mut path = root.clone();
        path.push_str(filename);

        if Path::new(&path).exists() {
            return Some(path);
        }
    }

    None
}


// The ".grp" file format is just a collection of a lot of files stored into 1 big
// one. I tried to make the format as simple as possible: The first 12 bytes
// contains my name, "KenSilverman". The next 4 bytes is the number of files that
// were compacted into the group file. Then for each file, there is a 16 byte
// structure, where the first 12 bytes are the filename, and the last 4 bytes are
// the file's size. The rest of the group file is just the raw data packed one
// after the other in the same order as the list of files.


/// Implementation of a group file "cache", into which the contents of various
/// group files are loaded. This is only somewhat reminiscent of the way
/// Silverman's original code goes about loading game data.

#[derive(Debug)]
pub struct GroupManager {
    files: HashMap<String, Vec<u8>>,
}

impl GroupManager {
    pub fn new() -> GroupManager {
        GroupManager { files: HashMap::new() }
    }

    /// Loads the contents of an in-memory group file into the cache.
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

            let data = data[data_off..data_off+size].to_vec();
            data_off += size;

            self.files.insert(name, data);
        }

        Ok(())
    }

    /// Loads the contents of an on-disk group file into the cache.
    pub fn load_file(&mut self, filename: &str) -> Result<(), Box<Error>> {
        let filename = match find_file(filename) {
            Some(filename) => filename,
            None => bail!("File not found in any search paths."),
        };

        let mut file = File::open(filename)?;
        let mut bytes: Vec<u8> = Vec::new();

        file.read_to_end(&mut bytes)?;
        self.load_data(&bytes)?;

        Ok(())
    }

    /// Obtains binary data associated with the given filename from the cache.
    pub fn get(&self, filename: &str) -> Option<&[u8]> {
        Some(&self.files.get(filename)?)
    }
}


#[cfg(test)]
mod tests {
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

        let mut group_manager = GroupManager::new();

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

        let mut group_manager = GroupManager::new();

        match group_manager.load_data(&data) {
            Ok(_) => panic!("Accepted incomplete header."),
            Err(_) => (),
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

        let mut group_manager = GroupManager::new();

        match group_manager.load_data(&data) {
            Ok(_) => panic!("Accepted invalid header."),
            Err(_) => (),
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

        let mut group_manager = GroupManager::new();

        match group_manager.load_data(&data) {
            Ok(_) => panic!("Accepted invalid header."),
            Err(_) => (),
        }
    }

    // TODO: Add checks for data_off and table_off going out of bounds.
}


// What's the .MAP / .ART file format?
//
// Go to my Build Source Code Page and download BUILDSRC.ZIP. I have a text file
// in there (BUILDINF.TXT) which describes both formats.


// What's the PALETTE.DAT format?
//
// See this separate PALETTE.TXT <http://advsys.net/ken/palette.txt> file which
// explains it all.


// What's the TABLES.DAT format ?
//
// See this separate TABLES.TXT <http://advsys.net/ken/tables.txt> file which
// explains it all.


// What's the .KVX file format?
//
// Go to my Projects Page <http://advsys.net/ken/download.htm?#slab6> and
// download SLAB6.ZIP. I have a text file in there (SLAB6.TXT) which describes
// the format.


// What's the .VOX file format?
//
// Both SLABSPRI & SLAB6 support a simpler, uncompressed voxel format using the
// .VOX file extension. (See the documentation that comes with those programs.)
// The .VOX format is simple enough to fit a description of it right here.
// Here's some C pseudocode:
//
// long xsiz, ysiz, zsiz;
// char voxel[xsiz][ysiz][zsiz];
// char palette[256][3];
//
// fil = open("?.vox",...);
// read(fil,&xsiz,4);
// read(fil,&ysiz,4);
// read(fil,&zsiz,4);
// read(fil,voxel,xsiz*ysiz*zsiz);
// read(fil,palette,768);
// close(fil);
//
// In the voxel array, use color 255 to define your empty space (air). For
// interior voxels (ones you can never see), do not use color 255, because it
// will prevent SLABSPRI from being able to take advantage of back-face culling.


// How does SLABSPRI convert images to voxels?
//
// It starts out with a solid cube. Then it runs through all of the rotations,
// chopping out any voxels that lie behind a transparent pixel (color 255). Once
// this is done, it runs through all the rotations again, this time painting
// colors onto the voxel object. If an individual cube is painted twice, the
// colors get averaged. Voxels that don't get hit by paint get randomly set to a
// nearby color.
