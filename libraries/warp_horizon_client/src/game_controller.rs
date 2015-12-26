use cgmath::Vector2;
pub use camera::{Camera};
pub use input_state::{InputState, GameButton};

trait WorldCoordinate {
    fn containing_tile(&self) -> Vector2<i32>;
}

impl WorldCoordinate for Vector2<f32> {
    fn containing_tile(&self) -> Vector2<i32> {
        Vector2::new(self.x.floor() as i32, self.y.floor() as i32)
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
        let tile_f = camera.screen_to_world(input_state.mouse_position());
        self.selected_tile = tile_f.containing_tile();
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
    fn selected_tile_returns_correct_tile_after_mouse_move_event() {
        let mut controller = GameController::new();
        let mut input_state = InputState::new();
        input_state.set_mouse_position(Vector2::new(50, 26 + 15));
        let camera = Camera::new(Vector2::new(100, 50));

        controller.update(&input_state, &camera);

        assert_eq!(controller.selected_tile(), Vector2::new(1, 1));
    }
}
