extern crate warp_horizon;

use warp_horizon::*;

fn main() {
    // Initialize the frontend
    let mut frontend = Frontend::init();

    // Initialize the game state
    let mut state = GameState { t: 0.0 };

    // Run the game loop
    let mut timer = FrameTimer::start();
    while !frontend.should_exit() {
        let delta = timer.tick();

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
