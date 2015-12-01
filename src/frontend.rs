use glium::{VertexBuffer, DisplayBuild, Surface, Program};
use glium::index::{NoIndices, PrimitiveType};
use glium::glutin::{WindowBuilder, Event};
use glium::backend::glutin_backend::GlutinFacade;
use nalgebra::OrthoMat3;
use GameState;

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
        let vertex_shader_src = r#"
            #version 140

            in vec2 position;

            uniform mat4 matrix;

            void main() {
                gl_Position = matrix * vec4(position, 0.0, 1.0);
            }
        "#;
        let fragment_shader_src = r#"
            #version 140

            out vec4 color;

            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;
        let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
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

    pub fn render(&self, state: &GameState) {
        // Load in the vertices
        let t = 0.0;
        let vertex1 = SimpleVertex { position: [-10.0 + t, -10.0] };
        let vertex2 = SimpleVertex { position: [0.0 + t, 10.0] };
        let vertex3 = SimpleVertex { position: [10.0 + t, -5.0] };
        let shape = vec![vertex1, vertex2, vertex3];
        let vertex_buffer = VertexBuffer::new(&self.display, &shape).unwrap();
        let indices = NoIndices(PrimitiveType::TrianglesList);

        // Create our projection matrix
        let matrix = OrthoMat3::<f32>::new(1280.0, 720.0, -10.0, 10.0);

        // Begin drawing
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        // Actually render our triangle
        let uniforms = uniform! {
            matrix: *matrix.as_mat().as_ref()
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
