extern crate byteorder;
extern crate simple_error;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;

use self::byteorder::{ByteOrder, LittleEndian};


// The ".grp" file format is just a collection of a lot of files stored into 1 big
// one. I tried to make the format as simple as possible: The first 12 bytes
// contains my name, "KenSilverman". The next 4 bytes is the number of files that
// were compacted into the group file. Then for each file, there is a 16 byte
// structure, where the first 12 bytes are the filename, and the last 4 bytes are
// the file's size. The rest of the group file is just the raw data packed one
// after the other in the same order as the list of files.

#[derive(Debug)]
pub struct GroupManager {
    files: HashMap<String, Vec<u8>>,
}

impl GroupManager {
    pub fn new() -> GroupManager {
        GroupManager { files: HashMap::new() }
    }

    pub fn load_from_slice(&mut self, data: &[u8]) -> Result<(), Box<Error>> {
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

    pub fn load_from_file(&mut self, filename: &str) -> Result<(), Box<Error>> {
        let mut file = File::open(filename)?;
        let mut bytes: Vec<u8> = Vec::new();

        file.read_to_end(&mut bytes)?;
        self.load_from_slice(&bytes)?;

        Ok(())
    }

    pub fn get(&self, filename: &str) -> Option<&[u8]> {
        Some(&self.files.get(filename)?)
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
