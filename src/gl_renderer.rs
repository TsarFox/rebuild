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

use self::glium::glutin;

use bitmap::BitmapManager;
use input::Event;

/// Renderer using the OpenGL library.
pub struct GLRenderer {
    events_loop: glutin::EventsLoop,
    display: glium::Display,
}

impl GLRenderer {
    /// Instantiate a new instance of the renderer.
    pub fn new(bitmaps: &BitmapManager) -> Result<GLRenderer, Box<Error>> {
        let _font = bitmaps.get_font("textfont").unwrap();

        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new();
        let context = glutin::ContextBuilder::new();
        let display = glium::Display::new(window, context, &events_loop)?;

        Ok(GLRenderer { events_loop, display })
    }

    // TODO: Document this.
    fn convert_event(ev: glutin::DeviceEvent) -> Event {
        match ev {
            // glutin::DeviceEvent::Button { button, state } => {
            //     if state == glutin::ElementState::Pressed {
            //         Event::KeyDown(button as u8)
            //     } else {
            //         Event::KeyUp(button as u8)
            //     }
            // },
            glutin::DeviceEvent::Key(press) => {
                Event::KeyDown(press.scancode as u8)
            }
            _ => Event::None,
        }
    }

    // TODO: Document this.
    pub fn handle_input<F>(&mut self, mut on_event: F)
    where F: FnMut(Event) {
        self.events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::DeviceEvent { device_id: _, event } => {
                    let equivalent = GLRenderer::convert_event(event);
                    on_event(equivalent);
                },
                _ => (),
            }
        });
    }
}
