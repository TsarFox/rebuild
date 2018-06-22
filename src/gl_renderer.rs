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

use bitmap::BitmapManager;

/// Renderer using the OpenGL library.
pub struct GLRenderer;

impl GLRenderer {
    /// Instantiate a new instance of the renderer.
    pub fn new(bitmaps: &BitmapManager) -> Result<GLRenderer, Box<Error>> {
        let _font = bitmaps.get_font("textfont").unwrap();

        Ok(GLRenderer { })
    }
}
