use cgmath::Vector2;
use glium::{VertexBuffer, Surface, Program, DrawParameters, Blend, Frame};
use glium::index::{NoIndices, PrimitiveType};
use glium::backend::glutin_backend::GlutinFacade;
use glium::draw_parameters::{BlendingFunction, LinearBlendingFactor, BackfaceCullingMode};
use glium::texture::srgb_texture2d::SrgbTexture2d;
use glium::uniforms::MagnifySamplerFilter;
use frontend::SimpleVertex;

pub struct TileBatch {
    vertices: Vec<SimpleVertex>,
}

impl TileBatch {
    pub fn new() -> Self {
        TileBatch { vertices: Vec::new() }
    }

    pub fn push_tile(&mut self,
                     pos: Vector2<f32>,
                     size: Vector2<f32>,
                     uv_start: Vector2<f32>,
                     uv_end: Vector2<f32>) {
        // I\
        self.vertices.push(SimpleVertex::new([pos.x, pos.y], [uv_start.x, uv_start.y]));
        self.vertices.push(SimpleVertex::new([pos.x + size.x, pos.y], [uv_end.x, uv_start.y]));
        self.vertices.push(SimpleVertex::new([pos.x, pos.y + size.y], [uv_start.x, uv_end.y]));

        // \I
        self.vertices.push(SimpleVertex::new([pos.x + size.x, pos.y], [uv_end.x, uv_start.y]));
        self.vertices
            .push(SimpleVertex::new([pos.x + size.x, pos.y + size.y], [uv_end.x, uv_end.y]));
        self.vertices.push(SimpleVertex::new([pos.x, pos.y + size.y], [uv_start.x, uv_end.y]));
    }

    pub fn draw(&self, matrix: &[[f32; 4]; 4], texture: &SrgbTexture2d, display: &GlutinFacade, program: &Program, target: &mut Frame) {
        // Turn the vertices into a VBO
        let vertex_buffer = VertexBuffer::dynamic(display, &self.vertices).unwrap();
        let indices = NoIndices(PrimitiveType::TrianglesList);

        // Set up the drawing parameters for these vertices
        let params = DrawParameters {
            blend: Blend {
                color: BlendingFunction::Addition {
                    source: LinearBlendingFactor::SourceAlpha,
                    destination: LinearBlendingFactor::OneMinusSourceAlpha,
                },
                ..Default::default()
            },
            backface_culling: BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        // Actually render the vertices
        let uniforms = uniform! {
            matrix: *matrix,
            tex: texture
                .sampled()
                .magnify_filter(MagnifySamplerFilter::Nearest),
        };
        target.draw(&vertex_buffer, &indices, &program, &uniforms, &params)
              .unwrap();
    }
}
