use std::thread::JoinHandle;
use std::sync::mpsc::{self, Sender, Receiver};
use glium::glutin::Event;
use tungsten::{Frontend, EventDispatcher};
use runtime::FrontendRuntime;

pub struct CloseRequestEvent;

pub trait View2D<M> {
    fn render(&mut self, model: &M, batch: &mut RenderBatch);
}

impl<M, F: Fn(&M, &mut RenderBatch)> View2D<M> for F {
    fn render(&mut self, model: &M, batch: &mut RenderBatch) {
        self(model, batch);
    }
}

pub struct Frontend2D<M> {
    views: Vec<Box<View2D<M>>>,
    _runtime_handle: JoinHandle<()>, // TODO: make sure the thread is told to gracefully stop
    event_recv: Receiver<Event>,
    batch_send: Sender<RenderBatch>,
    batch_return_recv: Receiver<RenderBatch>,
}

impl<M> Frontend2D<M> {
    pub fn new() -> Self {
        // Set up all the channels
        let (event_send, event_recv) = mpsc::channel();
        let (batch_send, batch_recv) = mpsc::channel();
        let (batch_return_send, batch_return_recv) = mpsc::channel();

        // Stick a single batch into the send-return loop to start out with
        batch_return_send.send(RenderBatch::new()).unwrap();

        // Start up the runtime
        let handle = FrontendRuntime::start(event_send, batch_recv, batch_return_send);

        Frontend2D {
            views: Vec::new(),
            _runtime_handle: handle,
            event_recv: event_recv,
            batch_send: batch_send,
            batch_return_recv: batch_return_recv,
        }
    }

    pub fn add_view<V: View2D<M> + 'static>(&mut self, view: V) {
        self.views.push(Box::new(view));
    }
}

impl<M: 'static> Frontend<M> for Frontend2D<M> {
    fn process_events(&mut self, dispatcher: &mut EventDispatcher<M>, model: &mut M) {
        // Process all received events
        loop {
            if let Ok(event) = self.event_recv.try_recv() {
                match event {
                    Event::Closed => dispatcher.dispatch(model, CloseRequestEvent),
                    _ => ()
                }
            } else {
                break;
            }
        }
    }

    fn render(&mut self, model: &M) {
        // Check if we receiver a batch back from the runtime
        let mut batch = {
            if let Ok(batch) = self.batch_return_recv.try_recv() {
                batch
            } else {
                // We didn't, don't render
                return;
            }
        };

        // Clear the batch before we continue to use it
        batch.clear();

        // Build up a render batch
        for view in &mut self.views {
            view.render(model, &mut batch);
        }

        // Send the batch to be rendered
        self.batch_send.send(batch).unwrap();
    }
}

pub struct Rectangle {
    pub position: [f32; 2],
    pub size: [f32; 2],
}

pub struct RenderBatch {
    rectangles: Vec<Rectangle>,
}

impl RenderBatch {
    fn new() -> Self {
        RenderBatch {
            rectangles: Vec::new()
        }
    }

    fn clear(&mut self) {
        self.rectangles.clear();
    }

    pub fn rectangle(&mut self, position: [f32; 2], size: [f32; 2]) {
        self.rectangles.push(Rectangle { position: position, size: size });
    }

    pub fn rectangles(&self) -> &Vec<Rectangle> {
        &self.rectangles
    }
}
