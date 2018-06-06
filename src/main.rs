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

mod grp;
mod path;
mod world;

fn main() {
    let path_manager = path::PathManager::new();

    let filename = "DUKE3D.GRP";
    let mut group_manager = grp::GroupManager::new(path_manager);

    if let Err(e) = group_manager.load_file(filename) {
        println!("Couldn't open {}: {}", filename, e);
        process::exit(1);
    }

    let map = group_manager.get("E1L1.MAP").unwrap();
    let world = world::World::from_map(map);
}
