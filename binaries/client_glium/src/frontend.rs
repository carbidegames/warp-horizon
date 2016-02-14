use glium::{self, DisplayBuild, Surface, DrawParameters, Program, VertexBuffer};
use glium::backend::glutin_backend::GlutinFacade;
use glium::draw_parameters::BackfaceCullingMode;
use glium::glutin::{Event, ElementState, VirtualKeyCode, WindowBuilder};
use glium::index::{NoIndices, PrimitiveType};
use cgmath::{PerspectiveFov, Matrix, Matrix4, Vector4, Angle, Rad, Deg, SquareMatrix};
use wavefront_obj::obj::{self, Object, Shape, VTNIndex};
use whc::{FrontendEvent, GameButton};
use client_state::ClientState;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, normal, color);

impl Vertex {
    fn from_wavefront(obj: &Object, indices: VTNIndex) -> Self {
        let (vi, _, ni) = indices;
        let v = obj.vertices[vi];
        let n = obj.normals[ni.unwrap()];

        Vertex {
            position: [v.x as f32, v.y as f32, v.z as f32],
            normal: [n.x as f32, n.y as f32, n.z as f32],
            color: [0.6, 0.6, 0.6]
        }
    }
}

pub struct Frontend {
    display: GlutinFacade,
    program: Program,
    vertex_buffer: VertexBuffer<Vertex>,
}

impl Frontend {
    pub fn init() -> Self {
        let display = WindowBuilder::new()
            .with_dimensions(1280, 720)
            .with_depth_buffer(24)
            .build_glium().unwrap();
        // The following is not yet implemented on Linux
        //display.get_window().unwrap().set_cursor_state(CursorState::Hide).unwrap();

        // Load in the ship
        let objs = obj::parse(include_str!("../assets/ship.obj").into()).unwrap();

        // Turn the file into vertices
        let mut vertices: Vec<Vertex> = Vec::new();
        for obj in &objs.objects {
            for shape in &obj.geometry[0].shapes {
                if let &Shape::Triangle(a, b, c) = shape {
                    vertices.push(Vertex::from_wavefront(obj, a));
                    vertices.push(Vertex::from_wavefront(obj, b));
                    vertices.push(Vertex::from_wavefront(obj, c));
                } else {
                    panic!("Non-triangle in obj file");
                }
            }
        }

        let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();

        let program = glium::Program::from_source(
            &display,
            include_str!("shader.vert.glsl"), include_str!("shader.frag.glsl"),
            None
        ).unwrap();

        Frontend {
            display: display,
            program: program,
            vertex_buffer: vertex_buffer,
        }
    }

    pub fn process_events(&mut self, buffer: &mut Vec<FrontendEvent>) {
        // Clear the buffer first
        buffer.clear();

        // Now go through all the events that are available
        for ev in self.display.poll_events() {
            match ev {
                Event::KeyboardInput(state, _scan_code, key_code) =>
                    Self::process_key(buffer, state, key_code),
                Event::MouseMoved(pos) => {
                    // Make an event out of the move
                    buffer.push(FrontendEvent::CursorMove(pos.into()));

                    // Center the cursor
                    self.display.get_window().unwrap().set_cursor_position(1280/2, 720/2).unwrap();
                },
                Event::Closed => {
                    buffer.push(FrontendEvent::Press(GameButton::RequestClose));
                    buffer.push(FrontendEvent::Release(GameButton::RequestClose));
                },
                _ => ()
            }
        }
    }

    fn process_key(
        buffer: &mut Vec<FrontendEvent>,
        state: ElementState, key: Option<VirtualKeyCode>
    ) {
        let key = match key.unwrap() {
            VirtualKeyCode::D => GameButton::MovePlayerRight,
            VirtualKeyCode::A => GameButton::MovePlayerLeft,
            VirtualKeyCode::W => GameButton::MovePlayerForward,
            VirtualKeyCode::S => GameButton::MovePlayerBackward,
            VirtualKeyCode::Escape => GameButton::RequestClose,
            _ => return // We don't know about this key, so just do nothing
        };
        let event = match state {
            ElementState::Pressed => FrontendEvent::Press(key),
            ElementState::Released => FrontendEvent::Release(key),
        };

        buffer.push(event);
    }

    pub fn render(&mut self, state: &ClientState) {
        let params = DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        let mut target = self.display.draw();
        target.clear_color_and_depth((0.01, 0.01, 0.01, 1.0), 1.0);

        // Calculate the matrices
        let perspective = Matrix4::from(PerspectiveFov {
            fovy: Rad::from(Deg::new(59.0)), // About 90 vertical fov for this aspect
            aspect: 1280.0/720.0,
            near: 0.1,
            far: 1000.0,
        });
        let projection_matrix: [[f32; 4]; 4] = perspective.into();

        let view = create_view_matrix(&state);
        let view_matrix: [[f32; 4]; 4] = view.into();

        // Calculate the light in view space
        let light_dir = Vector4::new(-1.0, 0.4, 0.9f32, 1.0);
        let light_dir_view = view.invert().unwrap().transpose() * light_dir;

        // Set up the uniforms
        let uniforms = uniform! {
            u_view: view_matrix,
            u_projection: projection_matrix,
            u_light_dir: [light_dir_view.x, light_dir_view.y, light_dir_view.z],
        };

        // Actually draw
        target.draw(
            &self.vertex_buffer, &NoIndices(PrimitiveType::TrianglesList), &self.program,
            &uniforms, &params
        ).unwrap();

        target.finish().unwrap();
    }
}

fn create_view_matrix(state: &ClientState) -> Matrix4<f32> {
    let translation = Matrix4::from_translation(state.player.position);
    let rotation = Matrix4::from(state.player.rotation);
    (translation * rotation).invert().unwrap()
}
