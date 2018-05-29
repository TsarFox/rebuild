extern crate byteorder;
extern crate simple_error;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use self::byteorder::{ByteOrder, LittleEndian};

// The ".grp" file format is just a collection of a lot of files stored into 1 big
// one. I tried to make the format as simple as possible: The first 12 bytes
// contains my name, "KenSilverman". The next 4 bytes is the number of files that
// were compacted into the group file. Then for each file, there is a 16 byte
// structure, where the first 12 bytes are the filename, and the last 4 bytes are
// the file's size. The rest of the group file is just the raw data packed one
// after the other in the same order as the list of files.

/// Implementation of a group file "cache", into which the contents of several
/// group files can be loaded. This is somewhat similar to the way that
/// Silverman's original code goes about loading game data. Additionally, this
/// struct manages a list of "search paths" on the filesystem to resolve the
/// absolute path of data files that have several possible locations.
/// See the documentation of 'load_file' for more information.
#[derive(Debug)]
pub struct GroupManager {
    files: HashMap<String, Vec<u8>>,
    search: Vec<String>,
}

impl GroupManager {
    pub fn new() -> GroupManager {
        let mut result = GroupManager {
            files: HashMap::new(),
            search: Vec::new()
        };

        result.init_search_paths();

        result
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

    /// Obtains binary data associated with the given filename from the cache.
    pub fn get(&self, filename: &str) -> Option<&[u8]> {
        Some(&self.files.get(filename)?)
    }

    /// Go through the search path list in order and load the contents of the
    /// first group archive with the given name. Specifically, the default
    /// search path order is:
    ///
    /// - ".",
    /// - "/usr/local/share/games/rebuild",
    /// - "/usr/share/games/rebuild",
    /// - "/usr/local/share/games/eduke32",
    /// - "/usr/share/games/eduke32",
    /// - "/usr/local/share/games/jfduke3d",
    /// - "/usr/share/games/jfduke3d",
    /// - "$HOME/.rebuild",
    ///
    /// This is followed by any additional paths in the search path list that
    /// came about from a call to 'add_search_path'.
    pub fn load_file(&mut self, filename: &str) -> Result<(), Box<Error>> {
        for directory in self.search.clone().iter() {
            let path = format!("{}/{}", directory, filename);

            if Path::new(&path).exists() {
                let mut file = File::open(path)?;
                let mut bytes: Vec<u8> = Vec::new();

                file.read_to_end(&mut bytes)?;
                self.load_data(&bytes)?;

                return Ok(());
            }
        }

        bail!("File not found in any search paths.")
    }

    /// Adds an additional path for 'load_file' to search.
    pub fn add_search_path(&mut self, path: &str) -> Result<(), Box<Error>> {
        if Path::new(&path).exists() {
            self.search.push(String::from(path));
        } else {
            bail!("Path does not exist");
        }

        Ok(())
    }

    // TODO: Conventional paths for OSX and Windows from EDuke32's
    // G_AddSearchPaths.
    fn init_search_paths(&mut self) {
        // Initial base paths that don't need a $HOME expansion.
        let directories = vec![
            ".",
            "/usr/share/games/jfduke3d",
            "/usr/local/share/games/jfduke3d",
            "/usr/share/games/eduke32",
            "/usr/local/share/games/eduke32",
            "/usr/share/games/rebuild",
            "/usr/local/share/games/rebuild",
        ];

        for directory in directories.iter() {
            self.add_search_path(directory).ok();
        }

        // TODO: Steam paths.
        let directories = vec![
            "$HOME/.rebuild",
        ];

        if let Some(home) = env::home_dir() {
            if let Some(home) = home.to_str() {
                for path in directories.iter() {
                    let path = String::from(*path).replace("$HOME", home);
                    self.add_search_path(&path).ok();
                }
            }
        }
    }
}


// What's the .MAP / .ART file format?
//
// Go to my Build Source Code Page and download BUILDSRC.ZIP. I have a text file
// in there (BUILDINF.TXT) which describes both formats.

// TODO: Write documentation
// pub struct Art {

// }


// impl Art {
//     pub fn new(data: &[u8]) -> Result<Art, Box<Error>> {
//         let len = data.len();
//         let version = LittleEndian::read_u32(&data[0..4]);

//         let _tile_count = LittleEndian::read_u32(&data[4..8]);
//         let first_tile = LittleEndian::read_u32(&data[8..12]);
//         let last_tile = LittleEndian::read_u32(&data[12..16]);
//         let tile_count = last_tile - first_tile + 1; // + 1?

//         let tiles_x: Vec<u16> = Vec::new();
//         let tiles_y: Vec<u16> = Vec::new();
//         let tiles_animation: Vec<u32> = Vec::new();

//         // short tilesizx[localtileend-localtilestart+1];
//         // short tilesizy[localtileend-localtilestart+1];

//         if version != 1 {
//             bail!("Invalid ART version");
//         }
//     }
// }


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
