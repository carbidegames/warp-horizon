use cgmath::Vector2;
use warp_horizon::Grid;
use camera::Camera;
use input_state::InputState;

trait WorldCoordinate {
    fn containing_tile(&self) -> Vector2<i32>;
}

impl WorldCoordinate for Vector2<f32> {
    fn containing_tile(&self) -> Vector2<i32> {
        Vector2::new(self.x.floor() as i32, self.y.floor() as i32)
    }
}

pub struct GridInputController {
    selected_tile: Option<Vector2<i32>>
}

impl GridInputController {
    pub fn new() -> Self {
        GridInputController {
            selected_tile: None
        }
    }

    pub fn update(&mut self, grid: &Grid, camera: &Camera, input_state: &InputState) {
        let tile_f = camera.screen_to_world(input_state.mouse_position());
        let tile = tile_f.containing_tile();

        self.selected_tile =
            if grid.get(tile).is_some() { Some(tile) }
            else { None };
    }

    pub fn selected_tile(&self) -> Option<Vector2<i32>> {
        self.selected_tile
    }
}

#[cfg(test)]
mod tests {
    use cgmath::Vector2;
    use warp_horizon::Grid;
    use camera::Camera;
    use grid_input::GridInputController;
    use input_state::InputState;

    #[test]
    fn selected_tile_returns_correct_tile_after_mouse_move_event() {
        let mut grid_input = GridInputController::new();
        let mut input_state = InputState::new();
        let camera = Camera::new(Vector2::new(100, 50));
        let grid = Grid::new(Vector2::new(10, 10));

        input_state.set_mouse_position(Vector2::new(50, 26 + 15));
        grid_input.update(&grid, &camera, &input_state);

        assert_eq!(grid_input.selected_tile(), Some(Vector2::new(1, 1)));
    }

    #[test]
    fn selected_tile_returns_none_if_out_of_bounds() {
        let mut grid_input = GridInputController::new();
        let mut input_state = InputState::new();
        let camera = Camera::new(Vector2::new(100, 50));
        let grid = Grid::new(Vector2::new(10, 10));

        input_state.set_mouse_position(Vector2::new(50, 10));
        grid_input.update(&grid, &camera, &input_state);

        assert_eq!(grid_input.selected_tile(), None);
    }
}
