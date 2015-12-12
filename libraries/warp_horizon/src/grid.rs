use rand::{XorShiftRng, SeedableRng, Rng};

/// Represents a single game grid.
pub struct Grid {
    tiles: Vec<i32>,
    width: i32,
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Self {
        let mut rng = XorShiftRng::from_seed([1, 2, 3, 4]);
        let mut tiles = vec![0i32; (width*height) as usize];

        for n in 0..tiles.len() {
            tiles[n] = (rng.next_u32() % 2) as i32;
        }

        Grid {
            tiles: tiles,
            width: width,
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.tiles.len() as i32 / self.width
    }

    pub fn get(&self, x: i32, y: i32) -> Option<i32> {
        self.tiles.get((x + y * self.width) as usize).map(|v| *v)
    }
}

#[test]
fn grid_creates_with_given_size() {
    let grid = Grid::new(40, 32);
    assert_eq!(grid.width(), 40);
    assert_eq!(grid.height(), 32);

    // Make sure we can actually access within that area
    assert!(grid.get(0, 0).is_some());
    assert!(grid.get(39, 31).is_some());
    assert!(grid.get(40, 32).is_none());
}
