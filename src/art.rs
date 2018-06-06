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

extern crate simple_error;

use std::error::Error;

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
//
//
// For canonical parsers, see:
// - 'paletteLoadFromDisk' in EDuke's 'build/src/palette.cpp'
// - 'loadpalette' in Build's 'ENGINE.C'

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
