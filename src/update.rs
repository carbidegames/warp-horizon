use std::sync::mpsc::{Sender};
use std::time::{Duration};
use std::thread;
use ::{GameState, EngineToken};

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
        //engine.submit_data(Box::new(state.clone()));
        sender.send(state.clone()).unwrap();

        // Wait for the next update step
        thread::sleep(Duration::from_millis(10));
    }
}
