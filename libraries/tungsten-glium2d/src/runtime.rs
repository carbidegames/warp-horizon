use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Barrier};
use std::thread::{self, JoinHandle};
use std::fs::File;
use cgmath::Matrix3;
use glium::backend::glutin_backend::GlutinFacade;
use glium::draw_parameters::DrawParameters;
use glium::glutin::{Event, WindowBuilder};
use glium::index::{NoIndices, PrimitiveType};
use glium::texture::RawImage2d;
use glium::texture::srgb_texture2d_array::SrgbTexture2dArray;
use glium::uniforms::MagnifySamplerFilter;
use glium::{Frame, DisplayBuild, Surface, VertexBuffer, Program, Blend};
use image;
use image::RgbaImage;
use {FrameRenderInfo, RenderBatchInfo, GameCameraInfo, RenderTarget, LayerInfo};
use frontend::{FrontendCommand};

#[derive(Copy, Clone)]
struct Vertex {
    i_position: [f32; 2],
    i_tex: [f32; 2],
    i_texid: f32,
}

implement_vertex!(Vertex, i_position, i_tex, i_texid);

pub struct FrontendRuntime {
    event_send: Sender<Event>,
    command_recv: Receiver<FrontendCommand>,
    batch_return_send: Sender<FrameRenderInfo>,

    display: GlutinFacade,
    program: Program,

    images: Vec<RgbaImage>,
    texture_array: Option<SrgbTexture2dArray>,
}

impl FrontendRuntime {
    pub fn start(
        event_send: Sender<Event>,
        command_recv: Receiver<FrontendCommand>, batch_return_send: Sender<FrameRenderInfo>
    ) -> JoinHandle<()> {
        let init_barrier = Arc::new(Barrier::new(2));
        let barrier_clone = init_barrier.clone();

        // Actually start the runtime thread
        let handle = thread::spawn(move || {
            let runtime = FrontendRuntime::new(event_send, command_recv, batch_return_send);
            runtime.run(barrier_clone);
        });

        // Wait for the runtime to be done initializing
        init_barrier.wait();

        handle
    }

    fn new(
        event_send: Sender<Event>,
        command_recv: Receiver<FrontendCommand>, batch_return_send: Sender<FrameRenderInfo>
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
            command_recv: command_recv,
            batch_return_send: batch_return_send,

            display: display,
            program: program,

            images: Vec::new(),
            texture_array: None,
        }
    }

    fn run(mut self, init_barrier: Arc<Barrier>) {
        init_barrier.wait();

        // Actually run the frontend loop
        loop {
            // Get the next queued command
            let command = self.command_recv.recv().unwrap();

            // Handle the command
            match command {
                FrontendCommand::Frame(frame) => {
                    // Check events for this frame
                    // We could do this separated from rendering but it's simpler if we just
                    // block on the recv.
                    for ev in self.display.poll_events() {
                        self.event_send.send(ev).unwrap();
                    }

                    // Render the frame
                    let glium_frame = self.render_frame(&frame);

                    // Return the batch and finish the frame (flipping the buffers)
                    self.batch_return_send.send(frame).unwrap();
                    glium_frame.finish().unwrap();
                },
                FrontendCommand::LoadTexture(id, path) => {
                    // Just verify the id is going to be right
                    assert!(self.images.len() as u32 == id.raw());

                    // Actually load and add the texture
                    let image_file = image::load(
                        File::open(path).unwrap(), image::PNG
                    ).unwrap().to_rgba();
                    self.images.push(image_file);

                    // Invalidate the texture array because of the new texture
                    //TODO: Allow texture unloading and re-use reclaimed space
                    self.texture_array = None;
                }
            }
        }
    }

    fn render_frame(&mut self, info: &FrameRenderInfo) -> Frame {
        // Create the texture array if needed
        if self.texture_array.is_none() {
            let mut images = Vec::new();

            for image_data in &self.images {
                let image_dimensions = image_data.dimensions();
                let image = RawImage2d::from_raw_rgba_reversed(
                    image_data.clone().into_raw(), image_dimensions
                );
                images.push(image);
            }

            self.texture_array = Some(SrgbTexture2dArray::new(&self.display, images).unwrap());
        }

        // Start a new frame
        let mut frame = self.display.draw();
        frame.clear_color(0.05, 0.05, 0.05, 1.0);

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

    fn render_camera(&mut self, frame: &mut Frame, camera: &GameCameraInfo) {
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

    fn render_batch(&mut self, frame: &mut Frame, camera: &GameCameraInfo, batch: &RenderBatchInfo) {
        // Get the texture array
        let texture_array = self.texture_array.as_ref().unwrap();

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
            m_matrix: matrix_raw,
            m_textures: texture_array.sampled().magnify_filter(MagnifySamplerFilter::Linear),
        };

        // TODO: Use a persistent memory mapped buffer
        let mut vertices = Vec::new();
        for rect in batch.rectangles() {
            let pos = &rect.position;
            let size = &rect.size;
            let size = [size[0] * 0.5, size[1] * 0.5];
            let rawid = rect.texture.raw() as f32;

            vertices.push(Vertex {
                i_position: [pos[0] - size[0], pos[1] - size[1]],
                i_tex: [0.0, 0.0], i_texid: rawid
            });
            vertices.push(Vertex {
                i_position: [pos[0] + size[0], pos[1] - size[1]],
                i_tex: [1.0, 0.0], i_texid: rawid
            });
            vertices.push(Vertex {
                i_position: [pos[0] + size[0], pos[1] + size[1]],
                i_tex: [1.0, 1.0], i_texid: rawid
            });

            vertices.push(Vertex {
                i_position: [pos[0] - size[0], pos[1] - size[1]],
                i_tex: [0.0, 0.0], i_texid: rawid
            });
            vertices.push(Vertex {
                i_position: [pos[0] + size[0], pos[1] + size[1]],
                i_tex: [1.0, 1.0], i_texid: rawid
            });
            vertices.push(Vertex {
                i_position: [pos[0] - size[0], pos[1] + size[1]],
                i_tex: [0.0, 1.0], i_texid: rawid
            });
        }
        let vertex_buffer = VertexBuffer::new(&self.display, &vertices).unwrap();
        let indices = NoIndices(PrimitiveType::TrianglesList);

        // Set up the draw parameters
        let params = DrawParameters {
            blend: Blend::alpha_blending(),
            .. Default::default()
        };

        // Actually do the draw call
        frame.draw(
            &vertex_buffer, &indices, &self.program,
            &uniforms, &params
        ).unwrap();
    }
}
