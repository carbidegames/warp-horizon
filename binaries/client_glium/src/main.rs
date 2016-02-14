extern crate cgmath;
#[macro_use] extern crate glium;
extern crate wavefront_obj;

mod client_state;
mod frontend;

use std::f32::consts;
use cgmath::{Vector2, Vector3, Angle, Rad, Quaternion, Rotation3};
use client_state::{ClientState, Player};
use frontend::{Frontend, FrontendEvent, GameButton};

fn main() {
    // Set up the frontend and an event buffer
    let mut frontend = Frontend::init();
    let mut buffer = Vec::new();

    // Set up the game's state
    let mut state = ClientState {
        player: Player {
            position: Vector3::new(0.0, 1.75, 5.0),
            rotation: Quaternion::one()
        }
    };

    loop {
        // Note: Investigate when we hit performance issues if we can get a performance benefit
        //  from making updating and rendering run on separate threads.
        // Process anything that happened in the frontend
        frontend.process_events(&mut buffer);

        // Update the game's state
        for e in &buffer {
            if let &FrontendEvent::Press(GameButton::RequestClose) = e {
                return;
            }

            if let &FrontendEvent::CursorMove(cursor) = e {
                let diff = cursor - Vector2::new(1280/2, 720/2);

                let (mut x, _, mut z) = state.player.rotation.to_euler();
                let lim = consts::PI/2.0-0.01;
                x = x + Rad::new(0.0005 * -diff.x as f32);
                z = limit(z + Rad::new(0.0005 * -diff.y as f32), -lim, lim);
                state.player.rotation = Quaternion::from_euler(x, Rad::new(0.0), z);
            }
        }

        // Render the new updated state
        frontend.render(&state);
    }
}

fn limit(value: Rad<f32>, min: f32, max: f32) -> Rad<f32> {
    Rad::new(f32::max(f32::min(value.s, max), min))
}
