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
use glium::uniforms::{Uniforms, MagnifySamplerFilter, UniformValue, Sampler, AsUniformValue};
use glium::{Frame, DisplayBuild, Surface, VertexBuffer, Program, Blend};
use image;
use {FrameRenderInfo, RenderBatchInfo, GameCameraInfo, RenderTarget, LayerInfo};
use frontend::{FrontendCommand, TextureId};

#[derive(Copy, Clone)]
struct Vertex2D {
    i_position: [f32; 2],
    i_texture_coord: [f32; 2],
    i_sampler_id: u32,
    i_texture_id: u32
}

implement_vertex!(Vertex2D, i_position, i_texture_coord, i_sampler_id, i_texture_id);

struct Uniforms2D<'a> {
    matrix: [[f32; 3]; 3],
    samplers: &'a [Sampler<'a, SrgbTexture2dArray>]
}

impl<'a> Uniforms for Uniforms2D<'a> {
    fn visit_values<'b, F: FnMut(&str, UniformValue<'b>)>(&'b self, mut func: F) {
        func("m_matrix", self.matrix.as_uniform_value());

        let mut i = 0;
        for sampler in self.samplers {
            func(&format!("m_samplers[{}]", i), sampler.as_uniform_value());
            i += 1;
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct TextureLocation {
    array: u32,
    index: u32,
}

type RawImageData = (Vec<u8>, (u32, u32));

struct Textures {
    sizes: Vec<u32>,
    id_registry: Vec<TextureLocation>, // The entries in here point to inside images and texture_array
    images: Vec<Vec<RawImageData>>, // This should match directly to texture_array's entries
    texture_arrays: Option<Vec<SrgbTexture2dArray>>,
}

impl Textures {
    fn new() -> Self {
        let sizes = vec!(16, 32, 64, 128, 256, 512, 1024, 2048);
        let images = vec![Vec::new(); sizes.len()];

        Textures {
            sizes: sizes,
            id_registry: Vec::new(),
            images: images,
            texture_arrays: None,
        }
    }

    fn load(&mut self, path: &str) -> u32 {
        // Load the image
        let image_file = image::load(
            File::open(path).unwrap(), image::PNG
        ).unwrap().to_rgba();

        // Get the texture data from the image
        let image_dimensions = image_file.dimensions();
        let image_raw = image_file.into_raw();

        // Make sure the data of the texture follows the requirements
        assert_eq!(image_dimensions.0, image_dimensions.1); // Square

        // Find the array to store the texture in, then store it
        let array = self.sizes.iter().position(|v| *v == image_dimensions.0)
            .expect("Texture must be power of two and 16x16 or higher.");
        let index = self.images[array].len();
        self.images[array].push((image_raw, image_dimensions));

        // Store a lookup for this texture
        let id = self.id_registry.len();
        self.id_registry.push(TextureLocation {
            array: array as u32,
            index: index as u32,
        });

        // Invalidate the texture arrays because of the new texture
        //TODO: Allow texture unloading and re-use reclaimed space
        self.texture_arrays = None;

        // Return the id of the texture
        id as u32
    }

    fn prepare_for_frame(&mut self, display: &GlutinFacade) {
        // If the texture arrays exists, we don't need to regenerate
        if self.texture_arrays.is_some() {
            return;
        }

        let mut arrays = Vec::new();

        // Go through all the images
        for image_array in &self.images {
            let mut textures = Vec::new();

            // Go through all the images for this array
            for image in image_array {
                let texture = RawImage2d::from_raw_rgba_reversed(
                    image.0.clone(), image.1
                );
                textures.push(texture);
            }

            // Create and store the texture array
            // If we have 0 textures, just create an empty one with space for one
            let array = if textures.len() != 0 {
                SrgbTexture2dArray::new(display, textures).unwrap()
            } else {
                SrgbTexture2dArray::empty(display, 1, 1, 1).unwrap()
            };
            arrays.push(array);
        }

        // Store the new texture arrays
        self.texture_arrays = Some(arrays);
    }

    fn samplers<'a>(&'a self) -> Vec<Sampler<'a, SrgbTexture2dArray>> {
        let arrays = self.texture_arrays.as_ref().unwrap();

        let mut samplers = Vec::new();
        for array in arrays {
            samplers.push(
                array
                    .sampled()
                    .magnify_filter(MagnifySamplerFilter::Linear)
            );
        }

        samplers
    }

    fn get(&self, id: TextureId) -> TextureLocation {
        self.id_registry[id.raw() as usize]
    }
}

pub struct FrontendRuntime {
    event_send: Sender<Event>,
    command_recv: Receiver<FrontendCommand>,
    batch_return_send: Sender<FrameRenderInfo>,

    display: GlutinFacade,
    program: Program,

    textures: Textures,
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

            textures: Textures::new(),
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
                    let loaded_at = self.textures.load(&path);
                    assert_eq!(loaded_at, id.raw());
                }
            }
        }
    }

    fn render_frame(&mut self, info: &FrameRenderInfo) -> Frame {
        // Prepare for the frame
        self.textures.prepare_for_frame(&self.display);

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
        let uniforms = Uniforms2D {
            matrix: matrix_raw,
            samplers: &self.textures.samplers(),
        };

        // Create all the vertices for the rectangles
        // TODO: Use a persistent memory mapped buffer
        let mut vertices = Vec::new();
        for rect in batch.rectangles() {
            let pos = &rect.position;
            let size = &rect.size;
            let size = [size[0] * 0.5, size[1] * 0.5];

            // Get the texture data
            let tex_data = self.textures.get(rect.texture);

            vertices.push(Vertex2D {
                i_position: [pos[0] - size[0], pos[1] - size[1]],
                i_texture_coord: [0.0, 0.0],
                i_sampler_id: tex_data.array, i_texture_id: tex_data.index,
            });
            vertices.push(Vertex2D {
                i_position: [pos[0] + size[0], pos[1] - size[1]],
                i_texture_coord: [1.0, 0.0],
                i_sampler_id: tex_data.array, i_texture_id: tex_data.index,
            });
            vertices.push(Vertex2D {
                i_position: [pos[0] + size[0], pos[1] + size[1]],
                i_texture_coord: [1.0, 1.0],
                i_sampler_id: tex_data.array, i_texture_id: tex_data.index,
            });

            vertices.push(Vertex2D {
                i_position: [pos[0] - size[0], pos[1] - size[1]],
                i_texture_coord: [0.0, 0.0],
                i_sampler_id: tex_data.array, i_texture_id: tex_data.index,
            });
            vertices.push(Vertex2D {
                i_position: [pos[0] + size[0], pos[1] + size[1]],
                i_texture_coord: [1.0, 1.0],
                i_sampler_id: tex_data.array, i_texture_id: tex_data.index,
            });
            vertices.push(Vertex2D {
                i_position: [pos[0] - size[0], pos[1] + size[1]],
                i_texture_coord: [0.0, 1.0],
                i_sampler_id: tex_data.array, i_texture_id: tex_data.index,
            });
        }

        // Turn the vertices into a vertex buffer
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
