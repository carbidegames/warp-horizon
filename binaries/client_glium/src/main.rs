extern crate cgmath;
#[macro_use] extern crate glium;
extern crate image;
extern crate warp_horizon;

mod frontend;

use warp_horizon::*;
use frontend::*;

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
        let events = frontend.process_events();

        // Update the game state
        state.update(delta, &events);

        // Render
        frontend.render(&state);
    }
}
