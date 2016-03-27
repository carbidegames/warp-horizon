use glium::glutin::{Event, WindowBuilder};
use glium::backend::glutin_backend::GlutinFacade;
use glium::{Frame, Surface};
use tungsten::{Frontend, EventDispatcher};

pub struct CloseRequestEvent;

pub trait GliumView<M> {
    fn render(&mut self, model: &M, frame: &mut Frame);
}

impl<M, F: Fn(&M, &mut Frame)> GliumView<M> for F {
    fn render(&mut self, model: &M, frame: &mut Frame) {
        self(model, frame);
    }
}

pub struct GliumFrontend<M> {
    display: GlutinFacade,
    views: Vec<Box<GliumView<M>>>,
}

impl<M> GliumFrontend<M> {
    pub fn new() -> Self {
        use glium::DisplayBuild;
        let display = WindowBuilder::new()
            .with_dimensions(1280, 720)
            .build_glium().unwrap();

        GliumFrontend {
            display: display,
            views: Vec::new()
        }
    }

    pub fn add_view<V: GliumView<M> + 'static>(&mut self, view: V) {
        self.views.push(Box::new(view));
    }
}

impl<M: 'static> Frontend<M> for GliumFrontend<M> {
    fn process_events(&mut self, dispatcher: &mut EventDispatcher<M>, model: &mut M) {
        for ev in self.display.poll_events() {
            match ev {
                Event::Closed => dispatcher.dispatch(model, CloseRequestEvent),
                _ => ()
            }
        }
    }

    fn render(&mut self, model: &M) {
        let mut frame = self.display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        for view in &mut self.views {
            view.render(model, &mut frame);
        }
        frame.finish().unwrap();
    }
}
