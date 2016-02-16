use std::f32::consts;
use cgmath::{Angle, EuclideanVector, Quaternion, Rad, Rotation3, Vector2, Vector3};
use time::Duration;
use frame_timer::UpdateDelta;
use frontend::{FrontendEvent, GameButton};
use input_state::InputState;

pub struct Player {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
}

pub struct WorldState  {
    pub player: Player,
}

impl WorldState {
    fn new() -> Self {
        WorldState {
            player: Player {
                position: Vector3::new(0.0, 1.75, 5.0),
                rotation: Quaternion::one()
            }
        }
    }
}

pub struct ClientState {
    input: InputState,
    world: WorldState,
    should_exit: bool,
}

impl ClientState {
    pub fn new() -> Self {
        ClientState {
            input: InputState::new(),
            world: WorldState::new(),
            should_exit: false,
        }
    }

    pub fn world(&self) -> &WorldState {
        &self.world
    }

    pub fn should_exit(&self) -> bool {
        self.should_exit
    }

    pub fn update(&mut self, delta: Duration, events: &[FrontendEvent]) {
        // Update the game's state
        self.input.update(&events);
        for e in events {
            if let &FrontendEvent::Press(GameButton::RequestClose) = e {
                self.should_exit = true;
            }

            if let &FrontendEvent::CursorMove(cursor) = e {
                let diff = cursor - Vector2::new(1280/2, 720/2);

                // Rotate the camera
                let (mut x, _, mut z) = self.world.player.rotation.to_euler();
                let lim = consts::PI/2.0-0.01;
                x = x + Rad::new(0.0005 * -diff.x as f32);
                z = limit(z + Rad::new(0.0005 * -diff.y as f32), -lim, lim);
                self.world.player.rotation = Quaternion::from_euler(x, Rad::new(0.0), z);
            }
        }

        // Process input state
        let mut amount = Vector2::new(0.0, 0.0);
        if self.input.key(GameButton::MovePlayerForward) { amount.x += 1.0; }
        if self.input.key(GameButton::MovePlayerBackward) { amount.x -= 1.0; }
        if self.input.key(GameButton::MovePlayerRight) { amount.y += 1.0; }
        if self.input.key(GameButton::MovePlayerLeft) { amount.y -= 1.0; }

        // Move the player
        let (x, _, _) = self.world.player.rotation.to_euler();
        let forward = Vector3::new(-x.sin(), 0.0, -x.cos());
        let right = Vector3::new(-forward.z, 0.0, forward.x);
        let direction = (forward * amount.x) + (right * amount.y);
        if f32::abs(direction.x) >= 0.01 || f32::abs(direction.y) >= 0.01 {
            let speed = 5.0;
            let normalized_direction = direction.normalize();
            self.world.player.position =
                self.world.player.position +
                delta.scale(normalized_direction * speed);
        }
    }
}

fn limit(value: Rad<f32>, min: f32, max: f32) -> Rad<f32> {
    Rad::new(f32::max(f32::min(value.s, max), min))
}
