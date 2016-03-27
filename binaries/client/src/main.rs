extern crate glium;
extern crate tungsten;
extern crate tungsten_glium2d;

use glium::Frame;
use tungsten::{Framework, EventDispatcher, UpdateEvent};
use tungsten_glium2d::{GliumFrontend, GliumView, CloseRequestEvent};

struct GameModel {
    should_close: bool,
    bird_height: f32
}

impl GameModel {
    fn new() -> Self {
        GameModel {
            should_close: false,
            bird_height: 4.0
        }
    }
}

fn close_request_handler(model: &mut GameModel, _event: &CloseRequestEvent) {
    model.should_close = true;
}

fn bird_update_handler(model: &mut GameModel, event: &UpdateEvent) {
    model.bird_height -= 1.0 * event.delta;
}

struct BirdView;

impl GliumView<GameModel> for BirdView {
    fn render(&mut self, model: &GameModel, _frame: &mut Frame) {
        println!("{}", model.bird_height);
    }
}

fn main() {
    let model = GameModel::new();

    let mut event_dispatcher = EventDispatcher::new();
    event_dispatcher.add_handler(close_request_handler);
    event_dispatcher.add_handler(bird_update_handler);

    let mut frontend = GliumFrontend::new();
    frontend.add_view(BirdView);

    let framework = Framework::new(model, frontend, event_dispatcher);
    framework.run(|model| !model.should_close);
}