mod camera;
mod input_state;

use cgmath::Vector2;
use time::Duration;
use Grid;
use self::input_state::{InputState, GameKey};

pub use self::camera::{Camera};

pub struct ClientState {
    main_grid: Grid,
    main_camera: Camera,
}

impl ClientState {
    pub fn new() -> Self {
        let mut cam = Camera::new();
        cam.set_position(Vector2::new(0.0, 0.0));
        cam.set_zoom(2);
        cam.set_move_speed(10.0);

        ClientState {
            main_grid: Grid::new(100, 100),
            main_camera: cam,
        }
    }

    pub fn update(&mut self, delta: Duration) {
        let mut state = InputState::new();
        state.set_key(GameKey::MoveCameraDown, true);
        self.main_camera.update(&state, delta);
    }

    pub fn main_grid(&self) -> &Grid {
        &self.main_grid
    }

    pub fn main_camera(&self) -> &Camera {
        &self.main_camera
    }
}
