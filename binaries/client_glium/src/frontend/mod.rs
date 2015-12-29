mod draw_batch;

use std::path::Path;
use cgmath;
use cgmath::Vector2;
use glium::{DisplayBuild, Surface, Program, Frame};
use glium::glutin::{WindowBuilder, Event, ElementState, VirtualKeyCode};
use glium::backend::glutin_backend::GlutinFacade;
use glium::texture::RawImage2d;
use glium::texture::srgb_texture2d::SrgbTexture2d;
use image;
use uv_utils::PixelsUv;
use warp_horizon::Grid;
use warp_horizon_client::{FrontendEvent, GameButton, ClientState, GridInputController, Camera};
use frontend::draw_batch::DrawBatch;

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

struct DrawResources {
    pub display: GlutinFacade,
    pub program: Program,
    pub texture: SrgbTexture2d,
}

struct FrameResources<'a> {
    pub target: &'a mut Frame,
    pub matrix: &'a [[f32; 4]; 4],
}

pub struct Frontend {
    resources: DrawResources,
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
        let raw_image = RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
        let texture = SrgbTexture2d::new(&display, raw_image).unwrap();

        // Create the frontend struct
        Frontend {
            resources: DrawResources {
                display: display,
                program: program,
                texture: texture,
            },
            should_exit: false,
        }
    }

    pub fn process_events(&mut self) -> Vec<FrontendEvent> {
        let mut events = Vec::new();

        // Poll all glutin events
        for ev in self.resources.display.poll_events() {
            match ev {
                Event::Closed => self.should_exit = true,
                Event::KeyboardInput(state, _scan_code, key_code) =>
                    Self::process_key(&mut events, state, key_code),
                Event::MouseMoved(pos) =>
                    events.push(FrontendEvent::MouseMove(pos.into())),
                _ => {},
            }
        }

        events
    }

    fn process_key(
        events: &mut Vec<FrontendEvent>,
        state: ElementState, key: Option<VirtualKeyCode>
    ) {
        let key = match key.unwrap() {
            VirtualKeyCode::Right => GameButton::MoveCameraRight,
            VirtualKeyCode::Left => GameButton::MoveCameraLeft,
            VirtualKeyCode::Up => GameButton::MoveCameraUp,
            VirtualKeyCode::Down => GameButton::MoveCameraDown,
            _ => return // We don't know about this key, so just do nothing
        };
        let event = match state {
            ElementState::Pressed => FrontendEvent::Press(key),
            ElementState::Released => FrontendEvent::Release(key),
        };

        events.push(event);
    }

    pub fn render(&self, state: &ClientState) {
        // Create our projection matrix
        // Our camera is a window onto an infinite plane, one pixel in graphics is one pixel on the
        // plane. We change the window defined in the matrix rather than changing the position of
        // vertices. The matrix however doesn't do the world-to-screen conversion, that's still
        // done when putting together the vertices, we just take away the worry of having to move
        // around depending on camera position away from those.
        let cam_pos = state.main_camera().position();
        let cam_scale = state.main_camera().zoom();
        let cam_half_res = Vector2::new((1280 / cam_scale) as f32, (720 / cam_scale) as f32) * 0.5;
        let matrix = cgmath::ortho(
            cam_pos.x - cam_half_res.x, cam_pos.x + cam_half_res.x,
            cam_pos.y - cam_half_res.y, cam_pos.y + cam_half_res.y,
            -10.0, 10.0
        );

        // Begin drawing
        let mut target = self.resources.display.draw();
        let mut brightness = 10.0 / 255.0;
        brightness = (((brightness+0.055)/1.055) as f32).powf(2.4); // sRGB to Linear
        target.clear_color(brightness, brightness, brightness, 1.0);

        {
            // Create the container struct for the frame's resources
            let mut frame = FrameResources {
                target: &mut target,
                matrix: &matrix.into()
            };

            // Draw the grid
            self.draw_grid(&mut frame, state.main_grid(), state.main_camera(), state.grid_input());
        }

        // Finish drawing
        target.finish().unwrap();
    }

    pub fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn draw_grid(&self, frame: &mut FrameResources, grid: &Grid, camera: &Camera, grid_input: &GridInputController) {
        // Some misc data about our tiles
        let tile_size = Vector2::new(32, 15);
        let tileset = PixelsUv::full([256, 120]);
        let ground_tile = tileset.subtexture([0, 0], tile_size.into()).to_opengl().correct_fp_error();
        let selection_tile = tileset.subtexture([0, 15], tile_size.into()).to_opengl().correct_fp_error();

        // Set up the tile batch we can use to draw
        let mut batch = DrawBatch::new();

        // Actually send over the tile data
        let size = grid.size();
        for y in 0..size.x {
            for x in 0..size.y {
                // If the tile is 0, there's nothing here
                if grid.get(Vector2::new(x, y)).unwrap() == 0 {
                    continue;
                }

                // Calculate the start of the grid cell this tile is in and where we have to draw
                let cell_start_pos = camera.world_to_renderplane(Vector2::new(x as f32, y as f32));
                //let pos = cell_start_pos - Vector2::new(tile_size.x as f32 * 0.5, tile_size.y as f32);
                let pos = cell_start_pos - (tile_size.cast() * Vector2::new(0.5, 1.0));

                // Add the tile to the batch
                batch.push_tile(
                    pos, tile_size.cast(),
                    ground_tile.start().into(), ground_tile.end().into()
                );
            }
        }

        // Draw the selection indicator if we have to
        if let Some(selection) = grid_input.selected_tile() {
            let cell_start_pos = camera.world_to_renderplane(selection.cast());
            let pos = cell_start_pos - (tile_size.cast() * Vector2::new(0.5, 1.0));
            batch.push_tile(
                pos, tile_size.cast(),
                selection_tile.start().into(), selection_tile.end().into()
            );
        }

        // Finally, draw
        batch.draw(&self.resources, frame);
    }
}
