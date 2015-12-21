use cgmath::Vector2;
use time::Duration;
use input_state::{InputState, GameButton};
use warp_horizon::UpdateDelta;

pub struct Camera {
    position: Vector2<f32>,
    zoom: i32,
    move_speed: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
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
    use time::Duration;
    use camera::Camera;
    use input_state::{InputState, GameButton};

    #[test]
    fn update_with_arrow_key_input_moves_camera() {
        let mut camera = Camera::new();
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
        let mut slow_cam = Camera::new();
        slow_cam.set_move_speed(1.0);
        let mut fast_cam = Camera::new();
        fast_cam.set_move_speed(2.0);

        let mut input_state = InputState::new();
        input_state.set_key(GameButton::MoveCameraRight, true);

        slow_cam.update(&input_state, Duration::milliseconds(20));
        fast_cam.update(&input_state, Duration::milliseconds(20));

        assert!(slow_cam.position().x < fast_cam.position().x);
    }
}
