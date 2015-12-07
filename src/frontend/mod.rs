use std::path::Path;
use glium::{VertexBuffer, DisplayBuild, Surface, Program, DrawParameters, Blend};
use glium::index::{NoIndices, PrimitiveType};
use glium::glutin::{WindowBuilder, Event};
use glium::backend::glutin_backend::GlutinFacade;
use glium::texture::RawImage2d;
use glium::texture::srgb_texture2d::SrgbTexture2d;
use glium::draw_parameters::{BlendingFunction, LinearBlendingFactor, BackfaceCullingMode};
use glium::uniforms::MagnifySamplerFilter;
use cgmath;
use cgmath::Vector2;
use image;
use ClientState;

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
        let program = Program::from_source(&display,
                                           include_str!("vert.glsl"),
                                           include_str!("frag.glsl"),
                                           None)
            .unwrap();

        // Load in the tileset
        let image = image::open(&Path::new("./assets/tiles.png"))
            .unwrap()
            .to_rgba();
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
        let matrix = cgmath::ortho(cam_pos.x,
                                   cam_pos.x + 1280.0,
                                   cam_pos.y,
                                   cam_pos.y + 720.0,
                                   -10.0,
                                   10.0);

        // Begin drawing
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        // Turn the map into vertices
        let grid = state.main_grid();
        let mut vertices: Vec<SimpleVertex> = Vec::new();
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if grid.get(x, y).unwrap() == 0 {
                    continue;
                }

                // Calculate some misc data about our tiles
                let scale = 2.0;
                let tile = Vector2::new(32.0, 15.0);
                let tiles = tile * scale;
                let uv = Vector2::new(1.0 / (256.0 / tile.x), 1.0 / (120.0 / tile.y));

                // Calculate the start of the grid cell this tile is in and where we have to draw
                let cell_start_pos = Vector2::new(x as f32, y as f32) * tiles;
                let pos = cell_start_pos - Vector2::new(tiles.x * 0.5, tiles.y);

                // I\
                vertices.push(SimpleVertex::new([pos.x, pos.y], [0.0, uv.y * 7.0]));
                vertices.push(SimpleVertex::new([pos.x + tiles.x, pos.y], [uv.x, uv.y * 7.0]));
                vertices.push(SimpleVertex::new([pos.x, pos.y + tiles.y], [0.0, uv.y * 8.0]));

                // \I
                vertices.push(SimpleVertex::new([pos.x + tiles.x, pos.y], [uv.x, uv.y * 7.0]));
                vertices.push(SimpleVertex::new([pos.x + tiles.x, pos.y + tiles.y],
                                                [uv.x, uv.y * 8.0]));
                vertices.push(SimpleVertex::new([pos.x, pos.y + tiles.y], [0.0, uv.y * 8.0]));
            }
        }

        // Turn the vertices into a VBO
        let vertex_buffer = VertexBuffer::dynamic(&self.display, &vertices).unwrap();
        let indices = NoIndices(PrimitiveType::TrianglesList);

        // Set up the drawing parameters for these vertices
        let params = DrawParameters {
            blend: {
                let mut blend: Blend = Default::default();
                blend.color = BlendingFunction::Addition {
                    source: LinearBlendingFactor::SourceAlpha,
                    destination: LinearBlendingFactor::OneMinusSourceAlpha,
                };
                blend
            },
            backface_culling: BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        // Actually render the vertices
        let uniforms = uniform! {
            matrix: { let m: [[f32; 4]; 4] = matrix.into(); m },
            tex: self.texture
                .sampled()
                .magnify_filter(MagnifySamplerFilter::Nearest),
        };
        target.draw(&vertex_buffer, &indices, &self.program, &uniforms, &params)
              .unwrap();

        // Finish drawing
        target.finish().unwrap();
    }

    pub fn should_exit(&self) -> bool {
        self.should_exit
    }
}
