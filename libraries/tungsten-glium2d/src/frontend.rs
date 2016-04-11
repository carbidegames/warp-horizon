use std::thread::JoinHandle;
use std::sync::mpsc::{self, Sender, Receiver};
use glium::glutin::Event;
use tungsten::{Frontend, EventDispatcher};
use runtime::FrontendRuntime;
use {Key, KeyState};

pub struct CloseRequestEvent;

pub struct KeyboardInputEvent {
    pub key: Key,
    pub state: KeyState,
}

pub trait View2D<M> {
    fn render(&mut self, model: &M, info: &mut FrameRenderInfo);
}

impl<M, F: Fn(&M, &mut FrameRenderInfo)> View2D<M> for F {
    fn render(&mut self, model: &M, info: &mut FrameRenderInfo) {
        self(model, info);
    }
}

pub enum FrontendCommand {
    Frame(FrameRenderInfo),
    LoadTexture(TextureId, String),
}

pub struct Frontend2D<M> {
    view: Option<Box<View2D<M>>>,
    _runtime_handle: JoinHandle<()>, // TODO: make sure the thread is told to gracefully stop

    event_recv: Receiver<Event>,
    command_send: Sender<FrontendCommand>,
    batch_return_recv: Receiver<FrameRenderInfo>,

    texture_id_counter: u32,
}

impl<M> Frontend2D<M> {
    pub fn new() -> Self {
        // Set up all the channels
        let (event_send, event_recv) = mpsc::channel();
        let (command_send, command_recv) = mpsc::channel();
        let (batch_return_send, batch_return_recv) = mpsc::channel();

        // Stick a single batch into the send-return loop to start out with
        batch_return_send.send(FrameRenderInfo::new()).unwrap();

        // Start up the runtime
        let handle = FrontendRuntime::start(event_send, command_recv, batch_return_send);

        Frontend2D {
            view: None,
            _runtime_handle: handle,

            event_recv: event_recv,
            command_send: command_send,
            batch_return_recv: batch_return_recv,

            texture_id_counter: 0,
        }
    }

    pub fn set_view<V: View2D<M> + 'static>(&mut self, view: V) {
        self.view = Some(Box::new(view));
    }

    pub fn load_texture(&mut self, path: &str) -> TextureId {
        let id = TextureId::from_raw(self.texture_id_counter);
        self.texture_id_counter += 1;

        let command = FrontendCommand::LoadTexture(id, path.into());
        self.command_send.send(command).unwrap();

        id
    }
}

impl<M: 'static> Frontend<M> for Frontend2D<M> {
    fn process_events(&mut self, dispatcher: &mut EventDispatcher<M>, model: &mut M) {
        // Process all received events
        loop {
            if let Ok(event) = self.event_recv.try_recv() {
                match event {
                    Event::Closed => dispatcher.dispatch(model, CloseRequestEvent),
                    Event::KeyboardInput(state, _, virtual_key) =>
                        dispatcher.dispatch(model, KeyboardInputEvent {
                            key: virtual_key.unwrap(),
                            state: state
                        }),
                    _ => ()
                }
            } else {
                break;
            }
        }
    }

    fn render(&mut self, model: &M) {
        // Check if we receiver a batch back from the runtime
        let mut frame = {
            if let Ok(frame) = self.batch_return_recv.try_recv() {
                frame
            } else {
                // We didn't, don't render
                return;
            }
        };

        // Clear the batch before we continue to use it
        frame.clear();

        // Build up a render batch
        self.view.as_mut().unwrap().render(model, &mut frame);

        // Send the batch to be rendered
        self.command_send.send(FrontendCommand::Frame(frame)).unwrap();
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TextureId {
    id: u32
}

impl TextureId {
    fn from_raw(id: u32) -> Self {
        TextureId {
            id: id
        }
    }

    pub fn raw(&self) -> u32 {
        self.id
    }
}

pub struct Rectangle {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub texture: TextureId,
}

pub enum LayerInfo {
    Camera(GameCameraInfo),
    Batch(RenderBatchInfo),
}

pub trait RenderTarget {
    fn game_camera(&mut self, position: [f32; 2]) -> &mut GameCameraInfo;
    fn batch(&mut self) -> &mut RenderBatchInfo;
    fn layers(&self) -> &Vec<LayerInfo>;
}

pub struct RenderBatchInfo {
    rectangles: Vec<Rectangle>
}

impl RenderBatchInfo {
    pub fn rectangle(&mut self, rect: Rectangle) {
        self.rectangles.push(rect);
    }

    pub fn rectangles(&self) -> &Vec<Rectangle> {
        &self.rectangles
    }
}

pub struct GameCameraInfo {
    position: [f32; 2],
    layers: Vec<LayerInfo>,
}

impl GameCameraInfo {
    pub fn position(&self) -> [f32; 2] {
        self.position
    }
}

impl RenderTarget for GameCameraInfo {
    fn game_camera(&mut self, _position: [f32; 2]) -> &mut GameCameraInfo {
        unimplemented!();
    }

    fn batch(&mut self) -> &mut RenderBatchInfo {
        let batch = RenderBatchInfo {
            rectangles: Vec::new()
        };
        self.layers.push(LayerInfo::Batch(batch));

        let last = self.layers.iter_mut().last().unwrap();
        if let &mut LayerInfo::Batch(ref mut batch) = last {
            return batch;
        }
        unreachable!();
    }

    fn layers(&self) -> &Vec<LayerInfo> {
        &self.layers
    }
}

pub struct FrameRenderInfo {
    layers: Vec<LayerInfo>,
}

impl FrameRenderInfo {
    fn new() -> Self {
        FrameRenderInfo {
            layers: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.layers.clear();
    }
}

impl RenderTarget for FrameRenderInfo {
    fn game_camera(&mut self, position: [f32; 2]) -> &mut GameCameraInfo {
        let cam = GameCameraInfo {
            position: position,
            layers: Vec::new(),
        };

        self.layers.push(LayerInfo::Camera(cam));

        let last = self.layers.iter_mut().last().unwrap();
        if let &mut LayerInfo::Camera(ref mut cam) = last {
            return cam;
        }
        unreachable!();
    }

    fn batch(&mut self) -> &mut RenderBatchInfo {
        unimplemented!();
    }

    fn layers(&self) -> &Vec<LayerInfo> {
        &self.layers
    }
}
