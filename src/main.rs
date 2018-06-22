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

#[macro_use]
extern crate simple_error;

use std::process;

mod bitmap;
mod gl_renderer;
mod grp;
mod path;
mod world;

fn main() {
    let paths = path::PathManager::new();

    let filename = "DUKE3D.GRP";
    let mut groups = grp::GroupManager::new(paths);

    if let Err(e) = groups.load_file(filename) {
        println!("Couldn't open {}: {}", filename, e);
        process::exit(1);
    }

    let map = groups.get("E1L1.MAP").unwrap();
    let _world = world::World::from_map(map).unwrap();

    let bitmaps = bitmap::BitmapManager::new(&groups).unwrap();
    let _tile = bitmaps.get_tile(0);

    let _renderer = gl_renderer::GLRenderer::new(&bitmaps);
}
