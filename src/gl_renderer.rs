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

extern crate glium;
extern crate simple_error;

use std::error::Error;

use bitmap::Bitmap;
use grp::GroupManager;

/// Renderer using the OpenGL library.
pub struct GLRenderer;

impl GLRenderer {
    pub fn new(groups: &GroupManager) -> Result<GLRenderer, Box<Error>> {
        let _font = GLRenderer::load_font(groups);

        Ok(GLRenderer { })
    }

    // TODO: Document this.
    // TODO: Wouldn't this make more sense in the 'bitmap' module?
    fn load_font(groups: &GroupManager) -> Result<Bitmap, Box<Error>> {
        let tables = match groups.get("TABLES.DAT") {
            Some(tables) => tables,
            None => bail!("GRP does not contain a TABLES.DAT"),
        };

        // <= ?
        if tables.len() < 7424 {
            bail!("TABLES.DAT does not contain a font.");
        }

        let font = &tables[5376..7424];

        let width = 128;
        let height = 256;
        let mut data = vec![0; height * width];

        // TODO: Replace '256' with a static 'MAX_GLYPH'.
        for glyph in 0..256 {
            let x_off = (glyph % 32) * 8;
            let y_off = (glyph / 32) * 8;

            for i in 0..8 {
                for j in 0..8 {
                    let byte = font[(glyph * 8 + i) as usize];
                    let bit = 2 << (7 - j);

                    if byte & bit != 0 {
                        // The font doesn't carry any color information, it's
                        // just white.
                        let pixel = 0xffffffff;

                        let x = x_off + i;
                        let y = y_off + j;

                        data[x * width + y] = pixel;
                    }
                }
            }
        }

        Ok(Bitmap { width: width as u16, height: height as u16, data })
    }
}
