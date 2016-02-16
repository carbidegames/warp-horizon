extern crate cgmath;
#[macro_use] extern crate glium;
extern crate wavefront_obj;
extern crate warphorizon_client as whc;

mod frontend;

use whc::{FrameTimer, ClientState};
use frontend::Frontend;

fn main() {
    // Set up the frontend and an event buffer
    let mut frontend = Frontend::init();
    let mut buffer = Vec::new();

    // Set up the game's state
    let mut state = ClientState::new();

    // Run the game loop
    let mut timer = FrameTimer::start();
    while !state.should_exit() {
        let delta = timer.tick();

        // Note: Investigate when we hit performance issues if we can get a performance benefit
        //  from making updating and rendering run on separate threads.

        // Process anything that happened in the frontend
        frontend.process_events(&mut buffer);

        // Pass the events to the client state
        state.update(delta, &buffer);

        // Render the new updated state
        frontend.render(&state);
    }
}
