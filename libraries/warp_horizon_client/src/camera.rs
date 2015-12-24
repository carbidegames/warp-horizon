use cgmath::Vector2;
use time::Duration;
use input_state::{InputState, GameButton};
use warp_horizon::UpdateDelta;

/// A type containing the data to convert between world and screen positions.
/// Also includes functionality for moving the camera around based on player input.
pub struct Camera {
    resolution: Vector2<i32>,
    position: Vector2<f32>,
    zoom: i32,
    move_speed: f32,
}

impl Camera {
    pub fn new(resolution: Vector2<i32>) -> Self {
        Camera {
            resolution: resolution,
            position: Vector2::new(0.0, 0.0),
            zoom: 1,
            move_speed: 1.0,
        }
    }

    pub fn position(&self) -> Vector2<f32> {
        self.position
    }

    pub fn set_position(&mut self, value: Vector2<f32>) {
        self.position = value;
    }

    pub fn zoom(&self) -> i32 {
        self.zoom
    }

    pub fn set_zoom(&mut self, value: i32) {
        self.zoom = value;
    }

    pub fn set_move_speed(&mut self, value: f32) {
        self.move_speed = value;
    }

    fn screen_to_renderplane(&self, value: Vector2<i32>) -> Vector2<f32> {
        // Get the distance from the center of the screen in screen coordinates
        let from_center: Vector2<f32> = (value - (self.resolution/2)).cast();

        // Calculate the distance from the center of the screen in renderplane coordinates and
        // add those to the position of the center
        self.position + (from_center / (self.zoom as f32))
    }

    fn renderplane_to_world(&self, value: Vector2<f32>) -> Vector2<f32> {
        // Misc trivial data before the actual calculation
        let tile = Vector2::new(32.0, 15.0);

        // The actual calculation just adds a certain amount to world-x and world-y per each
        // screen-x and screen-y, there's some room for optimization here if it becomes a problem
        // The +1 is needed on the y because there's 1 pixel of extra distance so the tiles
        // can slot together better
        let offset_from_x = Vector2::new(1.0, -1.0) * (value.x / tile.x);
        let offset_from_y = Vector2::new(1.0, 1.0) * (value.y / (tile.y+1.0));

        offset_from_x + offset_from_y
    }

    /// Calculates the equivalent coordinates in world for coordinates on screen.
    pub fn screen_to_world(&self, value: Vector2<i32>) -> Vector2<f32> {
        self.renderplane_to_world(self.screen_to_renderplane(value))
    }

    pub fn update(&mut self, state: &InputState, delta: Duration) {
        let mut direction = Vector2::new(0.0, 0.0);

        if state.key(GameButton::MoveCameraRight) { direction.x += 1.0; }
        if state.key(GameButton::MoveCameraLeft) { direction.x -= 1.0; }
        if state.key(GameButton::MoveCameraUp) { direction.y += 1.0; }
        if state.key(GameButton::MoveCameraDown) { direction.y -= 1.0; }

        self.position = self.position + (delta.scale(direction) * self.move_speed);
    }
}

#[cfg(test)]
mod tests {
    use cgmath::Vector2;
    use time::Duration;
    use camera::Camera;
    use input_state::{InputState, GameButton};

    #[test]
    fn update_with_arrow_key_input_moves_camera() {
        let mut camera = Camera::new(Vector2::new(100, 50));
        assert!(camera.position().x == 0.0);
        assert!(camera.position().y == 0.0);

        let mut input_state = InputState::new();
        input_state.set_key(GameButton::MoveCameraRight, true);
        input_state.set_key(GameButton::MoveCameraUp, true);

        camera.update(&input_state, Duration::milliseconds(20));

        assert!(camera.position().x > 0.0);
        assert!(camera.position().y > 0.0);
    }

    #[test]
    fn update_with_different_speeds_moves_at_different_rates() {
        let mut slow_cam = Camera::new(Vector2::new(100, 50));
        slow_cam.set_move_speed(1.0);
        let mut fast_cam = Camera::new(Vector2::new(100, 50));
        fast_cam.set_move_speed(2.0);

        let mut input_state = InputState::new();
        input_state.set_key(GameButton::MoveCameraRight, true);

        slow_cam.update(&input_state, Duration::milliseconds(20));
        fast_cam.update(&input_state, Duration::milliseconds(20));

        assert!(slow_cam.position().x < fast_cam.position().x);
    }

    #[test]
    fn screen_to_world_returns_correct_tile_at_origin_with_no_zoom() {
        let cam = Camera::new(Vector2::new(100, 50));

        // Origin
        let world1 = cam.screen_to_world(Vector2::new(50, 26));
        assert_eq!(world1.x.floor(), 0.0);
        assert_eq!(world1.y.floor(), 0.0);

        // Difference in the Y direction
        let world2 = cam.screen_to_world(Vector2::new(50, 26 + 16));
        assert_eq!(world2.x.floor(), 1.0);
        assert_eq!(world2.y.floor(), 1.0);

        // Difference in the X direction
        let world3 = cam.screen_to_world(Vector2::new(50 + 16, 26));
        assert_eq!(world3.x.floor(), 0.0);
        assert_eq!(world3.y.floor(), -1.0);

        // More complex position
        let world3 = cam.screen_to_world(Vector2::new(50 + 16, 26 + (16*20)));
        assert_eq!(world3.x.floor(), 20.0);
        assert_eq!(world3.y.floor(), 19.0);
    }

    #[test]
    fn screen_to_world_returns_correct_tile_with_zooming() {
        let mut cam = Camera::new(Vector2::new(100, 50));
        cam.set_zoom(4);

        // Origin
        let world1 = cam.screen_to_world(Vector2::new(50, 26));
        assert_eq!(world1.x.floor(), 0.0);
        assert_eq!(world1.y.floor(), 0.0);

        // Origin + 1 tile size down
        let world2 = cam.screen_to_world(Vector2::new(50, 26 + (16*4)));
        assert_eq!(world2.x.floor(), 1.0);
        assert_eq!(world2.y.floor(), 1.0);
    }

    #[test]
    fn screen_to_world_returns_correct_tile_with_position() {
        let mut cam = Camera::new(Vector2::new(100, 50));
        cam.set_position(Vector2::new(-16.0, 30.0));

        // Middle of screen
        let world1 = cam.screen_to_world(Vector2::new(50, 26));
        assert_eq!(world1.x.floor(), 1.0);
        assert_eq!(world1.y.floor(), 2.0);

        // Middle of screen + 1 tile size down
        let world2 = cam.screen_to_world(Vector2::new(50, 26 + 15));
        assert_eq!(world2.x.floor(), 2.0);
        assert_eq!(world2.y.floor(), 3.0);
    }
}
