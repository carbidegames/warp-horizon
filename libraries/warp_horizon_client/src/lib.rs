extern crate cgmath;
#[macro_use] extern crate enum_primitive;
extern crate time;
extern crate warp_horizon;

mod camera;
mod frontend;
mod input_state;

use cgmath::Vector2;
use time::Duration;
use warp_horizon::Grid;

pub use camera::{Camera};
pub use frontend::{FrontendEvent};
pub use input_state::{InputState, GameButton};

pub struct ClientState {
    main_grid: Grid,
    main_camera: Camera,
    input_state: InputState,
}

impl ClientState {
    pub fn new() -> Self {
        let mut cam = Camera::new(Vector2::new(1280, 720));
        cam.set_position(Vector2::new(0.0, 0.0));
        cam.set_zoom(2);
        cam.set_move_speed(80.0);

        ClientState {
            main_grid: Grid::new(100, 100),
            main_camera: cam,
            input_state: InputState::new()
        }
    }

    pub fn update(&mut self, delta: Duration, events: &[FrontendEvent]) {
        self.input_state.update(events);
        self.main_camera.update(&self.input_state, delta);
    }

    pub fn main_grid(&self) -> &Grid {
        &self.main_grid
    }

    pub fn main_camera(&self) -> &Camera {
        &self.main_camera
    }
}
