#[macro_use]
extern crate glium;
extern crate time;
extern crate nalgebra;
extern crate rand;

mod frontend;

use time::{PreciseTime, Duration};
use rand::{XorShiftRng, SeedableRng, Rng};
use nalgebra::{Vec2};

pub use frontend::Frontend;

pub trait TickDelta {
    fn scale(&self, value: f32) -> f32;
}

impl TickDelta for Duration {
    fn scale(&self, value: f32) -> f32 {
        value * (self.num_microseconds().unwrap() as f32 / 1_000_000.0)
    }
}

pub struct FrameTimer {
    last_time: PreciseTime,
}

impl FrameTimer {
    pub fn start() -> Self {
        FrameTimer { last_time: PreciseTime::now() }
    }

    pub fn tick(&mut self) -> Duration {
        let time = PreciseTime::now();
        let delta = self.last_time.to(time);
        self.last_time = time;

        delta
    }
}

/// Represents a single game grid.
pub struct Grid {
    tiles: Vec<i32>,
    width: i32
}

impl Grid {
    fn new(width: i32, height: i32) -> Self {
        let mut rng = XorShiftRng::from_seed([1, 2, 3, 4]);
        let mut tiles = vec![0i32; (width*height) as usize];

        for n in 0..tiles.len() {
            tiles[n] = (rng.next_u32()%2) as i32;
        }

        Grid {
            tiles: tiles,
            width: height,
        }
    }

    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.tiles.len() as i32 / self.width
    }

    fn at(&self, x: i32, y: i32) -> i32 {
        self.tiles[(x + y*self.width) as usize]
    }
}

pub struct Camera {
    position: Vec2<f32>
}

impl Camera {
    fn new() -> Self {
        Camera {
            position: Vec2::new(0.0, 0.0)
        }
    }

    pub fn position(&self) -> Vec2<f32> {
        self.position
    }
}

pub struct ClientState {
    main_grid: Grid,
    main_camera: Camera
}

impl ClientState {
    pub fn new() -> Self {
        ClientState {
            main_grid: Grid::new(100, 100),
            main_camera: Camera::new()
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
