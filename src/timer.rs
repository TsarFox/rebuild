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
