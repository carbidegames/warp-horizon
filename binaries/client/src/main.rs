extern crate tungsten;
extern crate tungsten_glium2d;

use tungsten::{Framework, EventDispatcher, UpdateEvent};
use tungsten_glium2d::{Frontend2D, CloseRequestEvent, RenderBatch};

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

    fn update(&mut self, delta: f32) {
        self.bird_height -= 1.0 * delta;
    }

    fn request_close(&mut self) {
        self.should_close = true;
    }

    fn keep_running(&self) -> bool {
        !self.should_close
    }
}

fn close_request_handler(model: &mut GameModel, _event: &CloseRequestEvent) {
    model.request_close();
}

fn bird_update_handler(model: &mut GameModel, event: &UpdateEvent) {
    model.update(event.delta);
}

fn bird_view(model: &GameModel, render: &mut RenderBatch) {
    render.rectangle([0.0, model.bird_height], [64.0, 64.0]);
}

fn main() {
    let model = GameModel::new();

    let mut event_dispatcher = EventDispatcher::new();
    event_dispatcher.add_handler(close_request_handler);
    event_dispatcher.add_handler(bird_update_handler);

    let mut frontend = Frontend2D::new();
    frontend.add_view(bird_view);

    let framework = Framework::new(model, frontend, event_dispatcher);
    framework.run(|model| model.keep_running());
}
