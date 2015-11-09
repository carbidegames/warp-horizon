#[macro_use]
extern crate glium;

mod glium_prelude;
use glium_prelude::*;

#[derive(Copy, Clone)]
struct SimpleVertex {
    position: [f32; 2],
}

implement_vertex!(SimpleVertex, position);

pub fn run_client() {
    // Set up our renderer
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    // Load in the vertices
    let vertex1 = SimpleVertex { position: [-0.5, -0.5] };
    let vertex2 = SimpleVertex { position: [ 0.0,  0.5] };
    let vertex3 = SimpleVertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];
    let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();
    let indices = NoIndices(PrimitiveType::TrianglesList);

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
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    // Mutable game state
    let mut t: f32 = 0.0;

    loop {
        // Poll all events
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }

        // Update game state
        t += 0.0002;
        if t > std::f32::consts::PI * 2.0 {
            t = 0.0;
        }

        // Begin drawing
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        // Actually render our triangle
        let uniforms = uniform! {
            matrix: [
                [ t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ]
        };
        target.draw(
            &vertex_buffer, &indices,
            &program,
            &uniforms, &Default::default()
        ).unwrap();

        // Finish drawing
        target.finish().unwrap();
    }
}
