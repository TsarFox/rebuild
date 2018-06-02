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

extern crate sdl2;

pub trait Renderer {
    pub fn new() -> Self;

    // FIXME: Error handling?
    pub fn window_title(title: &str);

    pub fn draw_rooms(player_num: i32, smooth_ratio: i32);
}

// TODO: Document this.
pub struct ClassicRenderer;

impl Renderer for ClassicRenderer {
    // TODO: Document this.
    // FIXME: Proper error handling.
    pub fn new() -> ClassicRenderer {
        // let context = sdl2::init().unwrap();
        // let video_subsystem = context.video().unwrap();

        // let display = video_subsystem.window("SDL2", 800, 600)
        //     // .resizable()
        //     // .build_glium()
        //     .unwrap();
        
        ClassicRenderer { }
    }

    pub fn window_title()
}
