use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Barrier};
use std::thread::{self, JoinHandle};
use cgmath::Matrix3;
use glium::backend::glutin_backend::GlutinFacade;
use glium::glutin::{Event, WindowBuilder};
use glium::index::{NoIndices, PrimitiveType};
use glium::{Frame, DisplayBuild, Surface, VertexBuffer, Program};
use {FrameRenderInfo, RenderBatchInfo, GameCameraInfo, RenderTarget, LayerInfo};

#[derive(Copy, Clone)]
struct Vertex {
    i_position: [f32; 2],
}

implement_vertex!(Vertex, i_position);

pub struct FrontendRuntime {
    event_send: Sender<Event>,
    batch_recv: Receiver<FrameRenderInfo>,
    batch_return_send: Sender<FrameRenderInfo>,

    display: GlutinFacade,
    program: Program,
}

impl FrontendRuntime {
    pub fn start(
        event_send: Sender<Event>,
        batch_recv: Receiver<FrameRenderInfo>, batch_return_send: Sender<FrameRenderInfo>
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
        batch_recv: Receiver<FrameRenderInfo>, batch_return_send: Sender<FrameRenderInfo>
    ) -> Self {
        let display = WindowBuilder::new()
            .with_dimensions(1280, 720)
            .with_title("Tungsten".into())
            .build_glium().unwrap();

        let program = Program::from_source(
            &display,
            include_str!("shader.vert.glsl"), include_str!("shader.frag.glsl"),
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
            if let Ok(frame) = self.batch_recv.try_recv() {
                // Render the frame
                let glium_frame = self.render_frame(&frame);

                // Return the batch and finish the frame (flipping the buffers)
                self.batch_return_send.send(frame).unwrap();
                glium_frame.finish().unwrap();
            }

            // Sleep a bit TODO: Only sleep if nothing was processed
            ::std::thread::sleep(::std::time::Duration::from_millis(1));
        }
    }

    fn render_frame(&self, info: &FrameRenderInfo) -> Frame {
        // Start a new frame
        let mut frame = self.display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);

        // Go over all the cameras
        // TODO: Support nested cameras
        for layer in info.layers() {
            if let &LayerInfo::Camera(ref camera) = layer {
                self.render_camera(&mut frame, camera);
            } else {
                unimplemented!();
            }
        }

        frame
    }

    fn render_camera(&self, frame: &mut Frame, camera: &GameCameraInfo) {
        // Go over all the batches
        // TODO: Support nested cameras
        for layer in camera.layers() {
            if let &LayerInfo::Batch(ref batch) = layer {
                self.render_batch(frame, camera, batch);
            } else {
                unimplemented!();
            }
        }
    }

    fn render_batch(&self, frame: &mut Frame, camera: &GameCameraInfo, batch: &RenderBatchInfo) {
        // Create the uniforms for the camera
        // TODO: Share between batches
        let proj_matrix: Matrix3<f32> = [
            [2.0/1280.0, 0.0, 0.0],
            [0.0, 2.0/720.0, 0.0],
            [0.0, 0.0, 1.0]
        ].into();

        let cam_pos = camera.position();
        let view_matrix: Matrix3<f32> = [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [-cam_pos[0], -cam_pos[1], 1.0]
        ].into();

        let matrix_raw: [[f32; 3]; 3] = (proj_matrix * view_matrix).into();
        let uniforms = uniform! {
            m_matrix: matrix_raw
        };

        // TODO: Use a persistent memory mapped buffer
        let mut vertices = Vec::new();
        for rect in batch.rectangles() {
            let pos = &rect.position;
            let size = &rect.size;
            let size = [size[0] * 0.5, size[1] * 0.5];

            vertices.push(Vertex { i_position: [pos[0] - size[0], pos[1] - size[1]] });
            vertices.push(Vertex { i_position: [pos[0] + size[0], pos[1] - size[1]] });
            vertices.push(Vertex { i_position: [pos[0] + size[0], pos[1] + size[1]] });

            vertices.push(Vertex { i_position: [pos[0] - size[0], pos[1] - size[1]] });
            vertices.push(Vertex { i_position: [pos[0] + size[0], pos[1] + size[1]] });
            vertices.push(Vertex { i_position: [pos[0] - size[0], pos[1] + size[1]] });
        }
        let vertex_buffer = VertexBuffer::new(&self.display, &vertices).unwrap();
        let indices = NoIndices(PrimitiveType::TrianglesList);
        frame.draw(
            &vertex_buffer, &indices, &self.program,
            &uniforms, &Default::default()
        ).unwrap();
    }
}
