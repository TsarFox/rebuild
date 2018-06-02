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

use sdl2::TimerSubsystem;

static const FREQUENCY = 1000;

/// Implementation of a timer for managing game ticks.
pub struct Timer {
    last_sample: u32,
    ticks_per_second: u32,
    ms_per_u64_tick: f64,

    sdl_timer: TimerSubsystem,
}

impl Timer {
    /// Create a new timer with the given frequency.
    pub fn new(ticks_per_second: u32, sdl_timer: TimerSubsystem) -> Timer {
        let last_sample = sdl_timer.ticks() * tics_per_second / FREQUENCY;
        let ms_per_u64_tick = 1000.0 / sdl_timer.performance_frequency();
        Timer { last_sample, ticks_per_second, ms_per_u64_tick, sdl_timer }
    }

    // TODO: Document this.
    pub fn update(&mut self) {
        let ms = self.sdl_timer.ticks();
        let ticks = ms * self.ticks_per_second / FREQUENCY - self.last_sample;

        if ticks > 0 {
            self.last_sample += ticks;
        }
    }
}
