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

use std::error::Error;
use std::io::{Cursor, Read, Seek, SeekFrom};

use self::byteorder::{LE, ReadBytesExt};
use grp::GroupManager;

/// A rectangular chunk of raw ARGB8888 data. That is, each byte carries 8 bits
/// of information about the intensity of a certain color. The following is a
/// list of bitmasks and which color ("channel," in the vernacular) they
/// represent.
///
/// 0x000000ff - Blue
/// 0x0000ff00 - Green
/// 0x00ff0000 - Red
/// 0xff000000 - Alpha (transparency)
///
/// # Examples
///
/// Colors are extracted from each individual integer via bitwise operations.
///
/// ```
/// let bitmap = bitmaps.get(277); // Arbitrary choice of tile number.
/// let corner = bitmap.data[0];
///
/// let b = corner & 0xff;
/// let g = (corner >> 8) & 0xff;
/// let r = (corner >> 16) & 0xff;
/// let a = (corner >> 24) & 0xff;
/// ```
#[derive(Clone)]
pub struct Bitmap {
    pub width: u16,
    pub height: u16,
    pub data: Vec<u32>,
}

/// Implementation of a bitmap cache, which is used for obtaining a bitmap
/// conversion of the individual tiles in a group file.
pub struct BitmapManager {
    bitmaps: Vec<Bitmap>,
}

impl BitmapManager {
    /// Create a new BitmapManager, loading all of the bitmap tiles in the given
    /// GRP archive.
    ///
    /// # Errors
    ///
    /// This will fail if no 'PALETTE.DAT' entry exists in the GRP archive, or
    /// if there is no 'TILES000.ART' entry.
    pub fn new(grp: &GroupManager) -> Result<BitmapManager, Box<Error>> {
        let mut bitmaps = Vec::new();

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
        // PALETTE: This 768 byte array is exactly the palette you want. The
        // format is: Red0, Green0, Blue0, Red1, Green1, Blue1, ..., Blue255
        //
        // The colors are based on the VGA 262,144 color palette. The values
        // range from 0-63, so if you want to convert it to a windows palette
        // you will have to multiply each byte by 4.
        //
        // NUMPALOOKUPS: The number of shading tables used. Usually this number
        // is 32, but 16 or 64 have also been used. Each of the 256 colors of
        // the VGA palette can take on any of "numpalookups" number of shades.
        //
        // PALOOKUP: The shading table. If numpalookups = 32, then this table
        // is: (32 shades) * (256 colors) = 8192 bytes (8K). The shade tables
        // are often made to go from normal brightness (shade #0) down to pitch
        // black (shade #31) So the first 256 bytes of the table would be for
        // shade #0, etc...
        //
        // TRANSLUC: 64K translucent lookup table. Given any 2 colors of the
        // palette, this lookup table gives the best match of the 2 colors when
        // mixed together.
        //
        // Here's a funny story: I noticed that Duke3D's PALETTE.DAT file is 8K
        // longer than it should be. Any PALETTE.DAT file with 32 shades and
        // translucent table should be 74,498 bytes. Duke3D's palette is 82,690
        // bytes, but it only has 32 shades! The reason is that at one time,
        // Duke3D had 64 shades in their "palookup" table. Then when we noticed
        // that this extra memory overhead slowed down the frame rate of the
        // game noticably, it was converted back to 32 shades. The problem is
        // that my palette conversion program never truncated off the end of the
        // file. So the last 8K of Duke3D's PALETTE.DAT is the last 8K of a
        // translucent table that was based on an older version of their
        // palette.
        //
        // For canonical parsers, see:
        // - 'paletteLoadFromDisk' in EDuke's 'build/src/palette.cpp'
        // - 'loadpalette' in Build's 'ENGINE.C'
        // - 'LoadPalette' in Transfusion's 'arttools/art2tga.c'

        let palette = if let Some(data) = grp.get("PALETTE.DAT") {
            let mut palette = Vec::new();

            for i in 0..256 {
                // The VGA 262,144 color palette appears darker than intended
                // when interpreted ARGB8888 output, so the intensity of the
                // red, green, and blue channels are scaled by a uniform factor.
                let a = (0xff) as u32;
                let r = (data[(i * 3)] << 2) as u32;
                let g = (data[(i * 3) + 1] << 2) as u32;
                let b = (data[(i * 3) + 2] << 2) as u32;
                palette.push(a << 24 | r << 16 | g << 8 | b);
            }

            palette
        } else {
            // FIXME: There should be a default palette to fall back on. Update
            // the documentation for this method when that's implemented.
            bail!("No PALETTE.DAT in GRP archive.");
        };

        // From BUILDINF.TXT
        //
        // All art files must have xxxxx###.ART. When loading an art file you
        // should keep trying to open new xxxxx###'s, incrementing the number,
        // until an art file is not found.

        for i in 0.. {
            if let Some(data) = grp.get(&format!("TILES{:03}.ART", i)) {
                // FIXME: Passing the Error back up the stack is suboptimal (?)
                let parsed = BitmapManager::load_art(data, &palette)?;
                bitmaps.extend_from_slice(&parsed);
            } else {
                // Indicates that we didn't even load TILES000.ART, meaning that
                // literally NO bitmaps were loaded. That's a pretty big issue.
                if i == 0 {
                    bail!("No TILES000.ART in GRP archive.");
                }

                // But if i > 0, we simply hit the last tilesheet. No problem.
                break;
            }
        }

        Ok(BitmapManager { bitmaps })
    }

    /// Load the tile given by the specified index, or None if no tile with the
    /// specified index exists.
    pub fn get(&self, index: i32) -> Option<&Bitmap> {
        let index = index as usize;

        if index < self.bitmaps.len() {
            Some(&self.bitmaps[index])
        } else {
            None
        }
    }

    /// Loads the tiles in an TILES###.ART file.
    fn load_art(data: &[u8], palette: &[u32]) -> Result<Vec<Bitmap>, Box<Error>> {
        let len = data.len() as u32;

        if len < 16 {
            bail!("ART does not contain a valid header.");
        }
        
        let mut data = Cursor::new(data);
        let mut bitmaps = Vec::new();

        // From BUILDINF.TXT
        //
        // 1. long artversion;
        //
        // The first 4 bytes in the art format are the version number. The
        // current current art version is now 1. If artversion is not 1 then
        // either it's the wrong art version or something is wrong.

        let version = data.read_u32::<LE>()?;

        if version != 1 {
            bail!("Invalid ART version.");
        }

        // 2. long numtiles;
        //
        // Numtiles is not really used anymore. I wouldn't trust it. Actually
        // when I originally planning art version 1 many months ago, I thought I
        // would need this variable, but it turned it is was unnecessary. To get
        // the number of tiles, you should search all art files, and check the
        // localtilestart and localtileend values for each file.

        let _count = data.read_u32::<LE>()?;

        // 3. long localtilestart;
        //
        // Localtilestart is the tile number of the first tile in this art file.
        //
        // 4. long localtileend;
        //
        // Localtileend is the tile number of the last tile in this art file.
        // Note: Localtileend CAN be higher than the last used slot in an art
        // file.
        //
        // Example:  If you chose 256 tiles per art file:
        // TILES000.ART -> localtilestart = 0,   localtileend = 255
        // TILES001.ART -> localtilestart = 256, localtileend = 511
        // TILES002.ART -> localtilestart = 512, localtileend = 767
        // TILES003.ART -> localtilestart = 768, localtileend = 1023

        let first_tile_index = data.read_u32::<LE>()?;
        let last_tile_index = data.read_u32::<LE>()?;
        let count = last_tile_index - first_tile_index + 1;

        if len < 8 * count + 16 {
            bail!(format!("Invalid number of tiles (given: {})", count));
        }

        // 5. short tilesizx[localtileend-localtilestart+1];
        //
        // This is an array of shorts of all the x dimensions of the tiles in this art
        // file. If you chose 256 tiles per art file then [localtileend-localtilestart+1]
        // should equal 256.
        //
        // 6. short tilesizy[localtileend-localtilestart+1];
        //
        // This is an array of shorts of all the y dimensions.
        //
        // 7. long picanm[localtileend-localtilestart+1];
        //
        // This array of longs stores a few attributes for each tile that you
        // can set inside EDITART. You probably won't be touching this array,
        // but I'll document it anyway.
        //
        // Bit:  |31           24|23           16|15            8|7             0|
        //       -----------------------------------------------------------------
        //       | | | | | | | | | | | | | | | | | | | | | | | | | | | | | | | | |
        //       -----------------------------------------------------------------
        //       | Anim. |  Signed char  |  Signed char  |   | Animate type:|
        //       | Speed |   Y-center    |   X-center    |   | 00 - NoAnm   |
        //       |-------|    offset     |    offset     |   | 01 - Oscil   |
        //               |---------------|---------------|   | 10 - AnmFd   |
        //                                                   | 11 - AnmBk   |
        //                                                   |--------------|
        //
        // You probably recognize these:
        // Animate speed - EDITART key: 'A', + and - to adjust
        // Signed char x&y offset - EDITART key: '`', Arrows to adjust
        // Animate number&type - EDITART key: +/- on keypad
        //
        // 8. After the picanm's, the rest of the file is straight-forward rectangular art
        // data. You must go through the tilesizx and tilesizy arrays to find where the
        // artwork is actually stored in this file.
        //
        // Note: The tiles are stored in the opposite coordinate system than the screen
        // memory is stored. Example on a 4*4 file:
        //
        // Offsets:
        // ---------------------
        // |  0 |  4 |  8 | 12 |
        // ---------------------
        // |  1 |  5 |  9 | 13 |
        // ---------------------
        // |  2 |  6 | 10 | 14 |
        // ---------------------
        // |  3 |  7 | 11 | 15 |
        // ---------------------

        // 16 bytes for the header, 2 bytes per entry for the array of bitmap
        // widths, 2 bytes per entry for the array of bitmap heights, and 4
        // bytes per entry for the array of bitmap attributes.

        let data_off = 16 + 2 * count + 2 * count + 4 * count;
        let mut data_off = data_off as u64;

        if (len as u64) < data_off {
            bail!(format!("Invalid number of tiles (given: {})", count));
        }

        for i in 0..count {
            let width_array_off = 2 * i + 16;
            let width_array_off = width_array_off as u64;
            data.seek(SeekFrom::Start(width_array_off))?;
            let width = data.read_u16::<LE>()? as usize;

            let height_array_off = 2 * i + 2 * count + 16;
            let height_array_off = height_array_off as u64;
            data.seek(SeekFrom::Start(height_array_off))?;
            let height = data.read_u16::<LE>()? as usize;

            let mut indices = vec![0; width * height];
            data.seek(SeekFrom::Start(data_off))?;
            data.read(&mut indices)?;
            data_off += indices.len() as u64;

            let mut data = Vec::new();

            for column in 0..width {
                for row in 0..height {
                    let index = indices[row * width + column];
                    data.push(palette[index as usize]);
                }
            }

            let width = width as u16;
            let height = height as u16;

            bitmaps.push(Bitmap { width, height, data });
        }

        Ok(bitmaps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_slice() {
        // Generated palette blob. All colors in the palette are white (HTML hex
        // ffffffff).
        let palette = vec![0xffffffff; 256];

        // Binary blob containing an ART test vector, made by me. Contains one
        // tile, a single black pixel.
        let data = vec![
            0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00,
        ];

        let parsed = BitmapManager::load_art(&data, &palette).unwrap();

        assert_eq!(parsed.len(), 1);

        let tile = &parsed[0];

        assert_eq!(tile.width, 1);
        assert_eq!(tile.height, 1);
        assert_eq!(tile.data[0], 0xffffffff);
    }

    #[test]
    fn test_incomplete_header() {
        let palette = vec![0xffffffff; 256];

        // Binary blob similar to the ART test vector above, but without the
        // actual bitmap data.
        let data = vec![
            0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        if let Ok(_) = BitmapManager::load_art(&data, &palette) {
            panic!("Parser accepted invalid ART file.");
        }
    }
}
