#[macro_use]
extern crate glium;
extern crate time;
extern crate nalgebra;

mod frontend;

use time::{PreciseTime, Duration};
use frontend::Frontend;

trait DeltaScale {
    fn scale(&self, value: f32) -> f32;
}

impl DeltaScale for Duration {
    fn scale(&self, value: f32) -> f32 {
        value * (self.num_microseconds().unwrap() as f32 / 1_000_000.0)
    }
}

pub struct GameState {
    t: f32,
}

pub fn run_client() {
    // Initialize the frontend
    let mut frontend = Frontend::init();

    // Initialize the game state
    let mut state = GameState { t: 0.0 };

    // Run the game loop
    let mut last_time = PreciseTime::now();
    while !frontend.should_exit() {
        // Get this tick's delta
        let time = PreciseTime::now();
        let delta = last_time.to(time);
        last_time = time;

        // Process events
        frontend.process_events();

        // Update the game state
        state.t = state.t + delta.scale(20.0);
        if state.t > 200.0 {
            state.t = 0.0;
        }

        // Render
        frontend.render(&state);
    }
}
