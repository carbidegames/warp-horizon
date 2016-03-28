use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Barrier};
use std::thread::{self, JoinHandle};
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::{Event, WindowBuilder};
use glium::index::{NoIndices, PrimitiveType};
use glium::uniforms::EmptyUniforms;
use glium::{DisplayBuild, Surface, VertexBuffer, Program};
use RenderBatch;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub struct FrontendRuntime {
    event_send: Sender<Event>,
    batch_recv: Receiver<RenderBatch>,
    batch_return_send: Sender<RenderBatch>,

    display: GlutinFacade,
    program: Program,
}

impl FrontendRuntime {
    pub fn start(
        event_send: Sender<Event>,
        batch_recv: Receiver<RenderBatch>, batch_return_send: Sender<RenderBatch>
    ) -> JoinHandle<()> {
        let init_barrier = Arc::new(Barrier::new(2));
        let barrier_clone = init_barrier.clone();

        // Actually start the runtime thread
        let handle = thread::spawn(move || {
            let runtime = FrontendRuntime::new(event_send, batch_recv, batch_return_send);
            runtime.run(barrier_clone);
        });

        // Wait for the runtime to be done initializing
        init_barrier.wait();

        handle
    }

    fn new(
        event_send: Sender<Event>,
        batch_recv: Receiver<RenderBatch>, batch_return_send: Sender<RenderBatch>
    ) -> Self {
        let display = WindowBuilder::new()
            .with_dimensions(1280, 720)
            .build_glium().unwrap();

        let vertex_shader_src = r#"
            #version 140

            in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;
        let fragment_shader_src = r#"
            #version 140

            out vec4 color;

            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;
        let program = Program::from_source(
            &display,
            vertex_shader_src, fragment_shader_src,
            None
        ).unwrap();

        FrontendRuntime {
            event_send: event_send,
            batch_recv: batch_recv,
            batch_return_send: batch_return_send,
            display: display,
            program: program,
        }
    }

    fn run(self, init_barrier: Arc<Barrier>) {
        init_barrier.wait();

        // Actually run the frontend loop
        loop {
            // Check events
            for ev in self.display.poll_events() {
                self.event_send.send(ev).unwrap();
            }

            // Check for frames to render
            if let Ok(batch) = self.batch_recv.try_recv() {
                // Start a new frame
                let mut frame = self.display.draw();
                frame.clear_color(0.0, 1.0, 0.0, 1.0);

                // Process the batch
                // TODO: persistent memory mapped buffer
                let mut vertices = Vec::new();
                for rect in batch.rectangles() {
                    let pos = &rect.position;
                    vertices.push(Vertex { position: [pos[0] - 0.5, pos[1] - 0.5] });
                    vertices.push(Vertex { position: [pos[0] + 0.5, pos[1] - 0.5] });
                    vertices.push(Vertex { position: [pos[0] + 0.0, pos[1] + 0.5] });
                    println!("{}", pos[1]);
                }
                let vertex_buffer = VertexBuffer::new(&self.display, &vertices).unwrap();
                let indices = NoIndices(PrimitiveType::TrianglesList);
                frame.draw(
                    &vertex_buffer, &indices, &self.program,
                    &EmptyUniforms, &Default::default()
                ).unwrap();

                // Return the batch and finish the frame
                self.batch_return_send.send(batch).unwrap();
                frame.finish().unwrap();
            }

            // Sleep a bit TODO: Only sleep if nothing was processed
            ::std::thread::sleep(::std::time::Duration::from_millis(1));
        }
    }
}
