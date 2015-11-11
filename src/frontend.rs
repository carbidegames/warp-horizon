use std::sync::mpsc::{Receiver};
use glium::{VertexBuffer, DisplayBuild, Surface, Program};
use glium::index::{NoIndices, PrimitiveType};
use glium::glutin::{WindowBuilder, Event};
use ::{GameState};

#[derive(Copy, Clone)]
struct SimpleVertex {
    position: [f32; 2],
}

implement_vertex!(SimpleVertex, position);

fn get_or_wait_state(receiver: &Receiver<GameState>) -> GameState {
    let mut prev_state = None;
    loop {
        // Non-blocking try to get state
        let state_r = receiver.try_recv();

        if let Ok(state) = state_r {
            // If we received a state, keep track of it while we continue checking
            prev_state = Some(state);
        }
        else {
            // There's no more states left in the recv
            if let Some(state) = prev_state {
                // We have had a state, so return it
                return state;
            }
            else {
                // We haven't had a state, so blocking wait
                return receiver.recv().unwrap();
            }
        }
    }
}

pub fn frontend_runtime(receiver: Receiver<GameState>) {
    // Set up our frontend
    let display = WindowBuilder::new().build_glium().unwrap();

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
    let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    // Run the render loop
    loop {
        // Wait for a game state
        let state = get_or_wait_state(&receiver);

        // Poll all events (TODO: pass them to the update runtime)
        for ev in display.poll_events() {
            match ev {
                Event::Closed => return,
                _ => ()
            }
        }

        // Begin drawing
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        // Actually render our triangle
        let t = state.t;
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
