extern crate byteorder;

use std::fmt;
use std::error::Error;

use self::byteorder::{ByteOrder, LittleEndian};

#[derive(Debug)]
pub struct FormatError {
    details: &'static str,
}

impl FormatError {
    fn new(details: &'static str) -> FormatError {
        FormatError { details }
    }
}

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for FormatError {
    fn description(&self) -> &str {
        self.details
    }
}


// The ".grp" file format is just a collection of a lot of files stored into 1 big
// one. I tried to make the format as simple as possible: The first 12 bytes
// contains my name, "KenSilverman". The next 4 bytes is the number of files that
// were compacted into the group file. Then for each file, there is a 16 byte
// structure, where the first 12 bytes are the filename, and the last 4 bytes are
// the file's size. The rest of the group file is just the raw data packed one
// after the other in the same order as the list of files.

#[derive(Debug)]
pub struct GroupEntry {
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct Group {
    pub file_count: usize,

    data: Vec<u8>,

    iter_index: usize,
    data_off: usize,
}

impl Group {
    pub fn new(data: &[u8]) -> Result<Group, Box<Error>> {
        let len = data.len();

        if len < 16 {
            let details = "'data' is too small to contain the GRP header.";
            return Err(Box::new(FormatError::new(details)));
        }

        let header = String::from_utf8(data[..12].to_vec())?;

        if header.as_str() != "KenSilverman" {
            let details = "Invalid GRP header.";
            return Err(Box::new(FormatError::new(details)));
        }

        let file_count = LittleEndian::read_u32(&data[12..16]) as usize;

        // 16 bytes for the header, and 16 bytes for each file entry. The raw
        // data will follow.
        let data_off = 16 * (file_count + 1) as usize;

        if data_off >= len {
            let details = "Invalid number of files.";
            return Err(Box::new(FormatError::new(details)));
        }

        Ok(Group { file_count, data: data.clone().to_vec(), iter_index: 0, data_off })
    }
}

impl Iterator for Group {
    type Item = GroupEntry;

    fn next(&mut self) -> Option<GroupEntry> {
        if self.iter_index >= self.file_count {
            return None;
        }

        let table_off = 16 * (1 + self.iter_index);

        // Raising an error would be more ideal than a sentinel filename.
        let size = LittleEndian::read_u32(&self.data[table_off+12..table_off+16]) as usize;
        let name = match String::from_utf8(self.data[table_off..table_off+12].to_vec()) {
            Ok(name) => name,
            Err(_) => String::from("ERRORFNAMEAA"),
        };

        let result = Some(GroupEntry { name, data: self.data[self.data_off..self.data_off+size].to_vec() });

        self.iter_index += 1;
        self.data_off += size;

        result
    }
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
