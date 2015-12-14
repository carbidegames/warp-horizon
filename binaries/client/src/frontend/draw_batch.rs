use cgmath::Vector2;
use glium::{VertexBuffer, Surface, DrawParameters, Blend};
use glium::index::{NoIndices, PrimitiveType};
use glium::draw_parameters::{BlendingFunction, LinearBlendingFactor, BackfaceCullingMode};
use glium::uniforms::MagnifySamplerFilter;
use frontend::{SimpleVertex, DrawResources, FrameResources};

pub struct DrawBatch {
    vertices: Vec<SimpleVertex>,
}

impl DrawBatch {
    pub fn new() -> Self {
        DrawBatch { vertices: Vec::new() }
    }

    pub fn push_tile(
        &mut self,
        pos: Vector2<f32>,
        size: Vector2<f32>,
        uv_start: Vector2<f32>,
        uv_end: Vector2<f32>
    ) {
        // I\
        self.vertices.push(SimpleVertex::new([pos.x, pos.y], [uv_start.x, uv_start.y]));
        self.vertices.push(SimpleVertex::new([pos.x + size.x, pos.y], [uv_end.x, uv_start.y]));
        self.vertices.push(SimpleVertex::new([pos.x, pos.y + size.y], [uv_start.x, uv_end.y]));

        // \I
        self.vertices.push(SimpleVertex::new([pos.x + size.x, pos.y], [uv_end.x, uv_start.y]));
        self.vertices.push(SimpleVertex::new([pos.x + size.x, pos.y + size.y], [uv_end.x, uv_end.y]));
        self.vertices.push(SimpleVertex::new([pos.x, pos.y + size.y], [uv_start.x, uv_end.y]));
    }

    pub fn draw(
        &self,
        resources: &DrawResources,
        frame: &mut FrameResources
    ) {
        // Turn the vertices into a VBO
        let vertex_buffer = VertexBuffer::dynamic(&resources.display, &self.vertices).unwrap();
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
            matrix: *frame.matrix,
            tex: resources.texture
                .sampled()
                .magnify_filter(MagnifySamplerFilter::Nearest),
        };
        frame.target.draw(&vertex_buffer, &indices, &resources.program, &uniforms, &params).unwrap();
    }
}
