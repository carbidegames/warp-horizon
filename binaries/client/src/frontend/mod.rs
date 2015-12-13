mod tile_batch;

use std::path::Path;
use cgmath;
use cgmath::Vector2;
use glium::{DisplayBuild, Surface, Program, Frame};
use glium::glutin::{WindowBuilder, Event};
use glium::backend::glutin_backend::GlutinFacade;
use glium::texture::RawImage2d;
use glium::texture::srgb_texture2d::SrgbTexture2d;
use image;
use warp_horizon::{ClientState, Grid};
use self::tile_batch::TileBatch;

#[derive(Copy, Clone)]
struct SimpleVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(SimpleVertex, position, tex_coords);

impl SimpleVertex {
    fn new(position: [f32; 2], tex_coords: [f32; 2]) -> Self {
        SimpleVertex {
            position: position,
            tex_coords: tex_coords,
        }
    }
}

pub struct Frontend {
    display: GlutinFacade,
    program: Program,
    texture: SrgbTexture2d,
    should_exit: bool,
}

impl Frontend {
    pub fn init() -> Self {
        // Set up our frontend
        let display = WindowBuilder::new()
            .with_dimensions(1280, 720)
            .build_glium()
            .unwrap();

        // Load in the shaders
        let program = Program::from_source(
            &display,
            include_str!("vert.glsl"),
            include_str!("frag.glsl"),
            None
        )
            .unwrap();

        // Load in the tileset
        let image = image::open(&Path::new("./assets/tiles.png")).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
        let texture = SrgbTexture2d::new(&display, image).unwrap();

        // Create the frontend struct
        Frontend {
            display: display,
            program: program,
            texture: texture,
            should_exit: false,
        }
    }

    pub fn process_events(&mut self) {
        // Poll all events
        for ev in self.display.poll_events() {
            match ev {
                Event::Closed => self.should_exit = true,
                _ => (),
            }
        }
    }

    pub fn render(&self, state: &ClientState) {
        // Create our projection matrix
        let cam_pos = state.main_camera().position();
        let matrix = cgmath::ortho(
            cam_pos.x, cam_pos.x + 1280.0,
            cam_pos.y, cam_pos.y + 720.0,
            -10.0, 10.0
        );

        // Begin drawing
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        // Draw the grid
        self.draw_grid(&matrix.into(), &mut target, state.main_grid());

        // Finish drawing
        target.finish().unwrap();
    }

    pub fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn draw_grid(&self, matrix: &[[f32; 4]; 4], target: &mut Frame, grid: &Grid) {
        // Set up the tile batch we can use to draw
        let mut batch = TileBatch::new();

        // Actually send over the tile data
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                // If the tile is 0, there's nothing here
                if grid.get(x, y).unwrap() == 0 {
                    continue;
                }

                // Calculate some misc data about our tiles
                let scale = 8.0;
                let tile = Vector2::new(32.0, 15.0);
                let tiles = tile * scale;
                let uv_per = Vector2::new(1.0 / (256.0 / tile.x), 1.0 / (120.0 / tile.y));

                // Calculate the start of the grid cell this tile is in and where we have to draw
                // the "+ 1.0*scale" bit is a hack to get it working, I don't know where the actual problem is
                let x_offset = Vector2::new(tiles.x * 0.5, -(tiles.y + 1.0*scale) * 0.5) * (x as f32);
                let y_offset = Vector2::new(-tiles.x * 0.5, -(tiles.y + 1.0*scale) * 0.5) * (y as f32);
                let cell_start_pos = x_offset + y_offset; // The start of the cell in world on screen
                let pos = cell_start_pos - Vector2::new(tiles.x * 0.5, tiles.y);

                // TODO Bug: tiles seem to be one pixel off in the y direction

                // Add the tile to the batch
                batch.push_tile(
                    pos, tiles,
                    Vector2::new(uv_per.x * 0.0, uv_per.y * 7.0),
                    Vector2::new(uv_per.x * 1.0, uv_per.y * 8.0)
                );
            }
        }

        // Finally, draw
        batch.draw(matrix, &self.texture, &self.display, &self.program, target);
    }
}
