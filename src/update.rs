use std::sync::mpsc::{Sender};
use ::{GameState};

pub fn update_runtime(sender: Sender<GameState>) {
    // Mutable game state
    let mut state = GameState {
        t: 0.0
    };

    // Run the update loop
    loop {
        // Update the game state
        state.t += 0.002;
        if state.t > ::std::f32::consts::PI * 2.0 {
            state.t = 0.0;
        }

        // Send the state to whoever needs it
        sender.send(state.clone()).unwrap();

        // Wait for the next update step
        ::std::thread::sleep_ms(10);
    }
}
