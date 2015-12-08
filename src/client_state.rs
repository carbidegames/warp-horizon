use time::Duration;
use cgmath::Vector2;
use Grid;

pub struct Camera {
    position: Vector2<f32>,
}

impl Camera {
    fn new() -> Self {
        Camera { position: Vector2::new(0.0, 0.0) }
    }

    pub fn position(&self) -> Vector2<f32> {
        self.position
    }

    pub fn set_position(&mut self, position: Vector2<f32>) {
        self.position = position;
    }
}

pub struct ClientState {
    main_grid: Grid,
    main_camera: Camera,
}

impl ClientState {
    pub fn new() -> Self {
        let mut cam = Camera::new();
        cam.set_position(Vector2::new(-500.0, -500.0));

        ClientState {
            main_grid: Grid::new(100, 100),
            main_camera: cam,
        }
    }

    pub fn update(&mut self, _delta: Duration) {
        // self.t = self.t + delta.scale(20.0);
        // if self.t > 200.0 {
        // self.t = 0.0;
        // }
    }

    pub fn main_grid(&self) -> &Grid {
        &self.main_grid
    }

    pub fn main_camera(&self) -> &Camera {
        &self.main_camera
    }
}