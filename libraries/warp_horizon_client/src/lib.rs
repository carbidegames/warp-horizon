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
    controller: GameController,
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
            input_state: InputState::new(),
            controller: GameController::new(),
        }
    }

    pub fn update(&mut self, delta: Duration, events: &[FrontendEvent]) {
        self.input_state.update(events);
        self.main_camera.update(&self.input_state, delta);
        self.controller.update(&self.input_state, &self.main_camera);
    }

    pub fn main_grid(&self) -> &Grid {
        &self.main_grid
    }

    pub fn main_camera(&self) -> &Camera {
        &self.main_camera
    }

    pub fn controller(&self) -> &GameController {
        &self.controller
    }
}

pub struct GameController {
    selected_tile: Vector2<i32>
}

impl GameController {
    pub fn new() -> Self {
        GameController {
            selected_tile: Vector2::new(0, 0)
        }
    }

    pub fn update(&mut self, input_state: &InputState, camera: &Camera) {
        self.selected_tile = camera.screen_to_world(input_state.mouse_position()).cast();
    }

    pub fn selected_tile(&self) -> Vector2<i32> {
        self.selected_tile
    }
}

#[cfg(test)]
mod tests {
    use cgmath::Vector2;
    use GameController;
    use camera::Camera;
    use input_state::InputState;

    #[test]
    fn controller_selected_tile_returns_correct_tile_after_mouse_move_event() {
        let mut controller = GameController::new();
        let mut input_state = InputState::new();
        input_state.set_mouse_position(Vector2::new(50, 26 + 15));
        let camera = Camera::new(Vector2::new(100, 50));

        controller.update(&input_state, &camera);

        assert_eq!(controller.selected_tile(), Vector2::new(1, 1));
    }
}
