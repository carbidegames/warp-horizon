use cgmath::Vector2;
use rand::{XorShiftRng, SeedableRng, Rng};

/// A representation of a single game grid.
pub struct Grid {
    tiles: Vec<i32>,
    size: Vector2<i32>,
}

impl Grid {
    pub fn new(size: Vector2<i32>) -> Self {
        let mut rng = XorShiftRng::from_seed([1, 2, 3, 4]);
        let mut tiles = vec![0i32; (size.x * size.y) as usize];

        for n in 0..tiles.len() {
            tiles[n] = (rng.next_u32() % 2) as i32;
        }

        Grid {
            tiles: tiles,
            size: size,
        }
    }

    pub fn size(&self) -> Vector2<i32> {
        self.size
    }

    pub fn get(&self, x: i32, y: i32) -> Option<i32> {
        // Sanity check the x and y
        if x < 0 || x >= self.size.x ||
           y < 0 || y >= self.size.y {
            return None;
        }

        // Actually perform the lookup
        self.tiles.get((x + y * self.size.x) as usize).map(|v| *v)
    }
}

#[cfg(test)]
mod tests {
    use cgmath::Vector2;
    use grid::Grid;

    #[test]
    fn new_creates_with_given_size() {
        let grid = Grid::new(Vector2::new(40, 32));
        assert_eq!(grid.size(), Vector2::new(40, 32));

        // Make sure we can actually access within that area
        assert!(grid.get(0, 0).is_some());
        assert!(grid.get(39, 31).is_some());
        assert!(grid.get(40, 32).is_none());
    }

    #[test]
    fn get_returns_none_on_invalid_tiles() {
        let grid = Grid::new(Vector2::new(10, 10));

        assert_eq!(grid.get(-1, -1), None);
        assert_eq!(grid.get(1, -1), None);
        assert_eq!(grid.get(-1, 1), None);
    }
}
