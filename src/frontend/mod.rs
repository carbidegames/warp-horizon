use glium::{VertexBuffer, DisplayBuild, Surface, Program};
use glium::index::{NoIndices, PrimitiveType};
use glium::glutin::{WindowBuilder, Event};
use glium::backend::glutin_backend::GlutinFacade;
use cgmath;
use ClientState;

#[derive(Copy, Clone)]
struct SimpleVertex {
    position: [f32; 2],
}

implement_vertex!(SimpleVertex, position);

pub struct Frontend {
    display: GlutinFacade,
    program: Program,
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

        // Create the frontend struct
        Frontend {
            display: display,
            program: program,
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
                                   cam_pos.y + 720.0,
                                   cam_pos.y + 0.0,
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

                let size = 32.0;
                let xf = x as f32 * size;
                let yf = y as f32 * size;
                vertices.push(SimpleVertex { position: [xf, yf] });
                vertices.push(SimpleVertex { position: [xf, yf + size] });
                vertices.push(SimpleVertex { position: [xf + size, yf] });

                vertices.push(SimpleVertex { position: [xf + size, yf] });
                vertices.push(SimpleVertex { position: [xf + size, yf + size] });
                vertices.push(SimpleVertex { position: [xf, yf + size] });
            }
        }

        // Turn the vertices into a VBO
        let vertex_buffer = VertexBuffer::dynamic(&self.display, &vertices).unwrap();
        let indices = NoIndices(PrimitiveType::TrianglesList);

        // Actually render the vertices
        let uniforms = uniform! {
            matrix: { let m: [[f32; 4]; 4] = matrix.into(); m }
        };
        target.draw(&vertex_buffer,
                    &indices,
                    &self.program,
                    &uniforms,
                    &Default::default())
              .unwrap();

        // Finish drawing
        target.finish().unwrap();
    }

    pub fn should_exit(&self) -> bool {
        self.should_exit
    }
}
