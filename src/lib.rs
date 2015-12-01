#[macro_use]
extern crate glium;
extern crate time;
extern crate nalgebra;

mod frontend;

use time::{PreciseTime, Duration};

pub use frontend::Frontend;

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
    pub fn start() -> Self {
        FrameTimer { last_time: PreciseTime::now() }
    }

    pub fn tick(&mut self) -> Duration {
        let time = PreciseTime::now();
        let delta = self.last_time.to(time);
        self.last_time = time;

        delta
    }
}

pub struct GameState {
    tiles: Vec<i32>,
    width: i32,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            tiles: vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            width: 4,
        }
    }

    pub fn update(&mut self, _delta: Duration) {
        // self.t = self.t + delta.scale(20.0);
        // if self.t > 200.0 {
        // self.t = 0.0;
        // }
    }
}
