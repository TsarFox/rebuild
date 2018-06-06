// Copyright (C) 2018 Jakob L. Kreuze, All Rights Reserved.
//
// This file is part of rebuild.
//
// rebuild is free software: you can redistribute it and/or modify it under the
// terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// rebuild is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
// A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// rebuild. If not, see <http://www.gnu.org/licenses/>.

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
