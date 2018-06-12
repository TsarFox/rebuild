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

/// Rectangular chunk of ARGB data.
#[derive(Clone)]
pub struct Bitmap {
    pub width: u16,
    pub height: u16,
    pub data: Vec<u32>,
}

/// Manages access to bitmap tiles in a GRP archive.
pub struct BitmapManager {
    bitmaps: Vec<Bitmap>,
}

impl BitmapManager {
    // FIXME: Should we have a hardcoded palette to fall back on if there is no
    // PALETTE.DAT? That might make sense for Blood.
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

            // FIXME: As of right now it's RGB, not ARGB.
            for i in 0..256 {
                let r = data[(i * 3)] as u32;
                let g = data[(i * 3) + 1] as u32;
                let b = data[(i * 3) + 2] as u32;

                // FIXME: Also, I'm not a fan of this manual shifting shit.
                palette.push(r << 4 | g << 2 | b << 0);
            }

            palette
        } else {
            bail!("No PALETTE.DAT");
        };

        // From BUILDINF.TXT
        //
        // All art files must have xxxxx###.ART. When loading an art file you
        // should keep trying to open new xxxxx###'s, incrementing the number,
        // until an art file is not found.
        for i in 0.. {
            if let Some(data) = grp.get(&format!("TILES{:03}.ART", i)) {
                // FIXME: Hoo, boy. Passing that Error back up the stack is
                // probably a pretty bad way of handling an invalid header.
                bitmaps.extend_from_slice(&BitmapManager::load_art(data, &palette)?);
            } else {
                // Indicates that there was no TILES000.ART, implying that
                // literally NO bitmaps were loaded. That probably isn't right.
                if i == 0 {
                    bail!("No TILES000.ART");
                }

                // But if we get here, it means that we just hit the last
                // tilesheet and it's not worth our time to continue checking.
                break;
            }
        }

        Ok(BitmapManager { bitmaps })
    }

    /// Load the tile given by the specified index, or None if no tile with the
    /// specified index exists.
    pub fn get_tile(&self, index: i32) -> Option<&Bitmap> {
        if (index as usize) < self.bitmaps.len() {
            Some(&self.bitmaps[(index as usize)])
        } else {
            None
        }
    }

    /// Loads the tiles in an TILES###.ART file.
    fn load_art(data: &[u8], palette: &[u32]) -> Result<Vec<Bitmap>, Box<Error>> {
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
        let _ = data.read_u32::<LE>()?;

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
        // -----------------
        // | 0 | 4 | 8 |12 |
        // -----------------
        // | 1 | 5 | 9 |13 |
        // -----------------
        // | 2 | 6 |10 |14 |
        // -----------------
        // | 3 | 7 |11 |15 |
        // -----------------
        let mut data_off = (16 + 2 * count + 2 * count + 4 * count) as u64;
        
        for i in 0..count {
            // The header is 16 bytes, the width and height arrays are both 2
            // bytes per value, and the attribute array is 4 bytes per value.
            // The raw bitmap data follows immediately and the bounds of each
            // bitmap are inferred from the width and height.
            let width_array_off = (2 * i + 16) as u64;
            let height_array_off = (2 * i + 2 * count + 16) as u64;

            data.seek(SeekFrom::Start(width_array_off))?;
            let width = data.read_u16::<LE>()?;

            data.seek(SeekFrom::Start(height_array_off))?;
            let height = data.read_u16::<LE>()?;

            let mut indices = vec![0; (width as usize) * (height as usize)];
            data.seek(SeekFrom::Start(data_off))?;
            data.read(&mut indices)?;
            data_off += indices.len() as u64;

            let mut data = Vec::new();

            // FIXME: Casting this as usize is just nasty, man.
            for column in 0..width {
                for row in 0..height {
                    let index = indices[(row as usize) * (width as usize) + (column as usize)];
                    data.push(palette[index as usize]);
                }
            }

            bitmaps.push(Bitmap { width, height, data });
        }

        Ok(bitmaps)
    }
}
