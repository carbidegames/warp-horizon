extern crate warp_horizon;

use warp_horizon::*;

fn main() {
    // Initialize the frontend
    let mut frontend = Frontend::init();

    // Initialize the game state
    let mut state = ClientState::new();

    // Run the game loop
    let mut timer = FrameTimer::start();
    while !frontend.should_exit() {
        let delta = timer.tick();

        // Process events
        frontend.process_events();

        // Update the game state
        state.update(delta);

        // Render
        frontend.render(&state);
    }
}
