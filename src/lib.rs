#[macro_use]
extern crate glium;
extern crate time;
extern crate nalgebra;

mod frontend;

use time::{PreciseTime, Duration};

pub use frontend::{Frontend};

pub trait TickDelta {
    fn scale(&self, value: f32) -> f32;
}

impl TickDelta for Duration {
    fn scale(&self, value: f32) -> f32 {
        value * (self.num_microseconds().unwrap() as f32 / 1_000_000.0)
    }
}

pub struct FrameTimer {
    last_time: PreciseTime,
}

impl FrameTimer {
    pub fn start() -> FrameTimer {
        FrameTimer {
            last_time: PreciseTime::now()
        }
    }

    pub fn tick(&mut self) -> Duration {
        let time = PreciseTime::now();
        let delta = self.last_time.to(time);
        self.last_time = time;

        delta
    }
}

pub struct GameState {
    pub t: f32,
}
