extern crate tungsten;
extern crate tungsten_glium2d;

use tungsten::{Framework, EventDispatcher, UpdateEvent};
use tungsten_glium2d::{Frontend2D, CloseRequestEvent, RenderBatch, KeyboardInputEvent, Key, KeyState};

struct GameModel {
    should_close: bool,
    bird_height: f32,
    bird_velocity: f32
}

impl GameModel {
    fn new() -> Self {
        GameModel {
            should_close: false,
            bird_height: 64.0,
            bird_velocity: 0.0,
        }
    }

    fn update(&mut self, delta: f32) {
        self.bird_velocity -= 3.0 * delta;
        self.bird_height += self.bird_velocity;

        // If the bird falls below this, the game is lost
        if self.bird_height < -300.0 {
            self.bird_height = 64.0;
            self.bird_velocity = 0.0;
        }
    }

    fn on_up_pressed(&mut self) {
        self.bird_velocity = 12.0;
    }

    fn on_request_close(&mut self) {
        self.should_close = true;
    }

    fn keep_running(&self) -> bool {
        !self.should_close
    }
}

fn close_request_handler(model: &mut GameModel, _event: &CloseRequestEvent) {
    model.on_request_close();
}

fn model_update_handler(model: &mut GameModel, event: &UpdateEvent) {
    model.update(event.delta);
}

fn keyboard_handler(model: &mut GameModel, event: &KeyboardInputEvent) {
    if event.state == KeyState::Pressed {
        if event.key == Key::Up {
            model.on_up_pressed();
        }
    }
}

fn bird_view(model: &GameModel, render: &mut RenderBatch) {
    render.rectangle([0.0, model.bird_height], [64.0, 64.0]);
}

fn main() {
    let model = GameModel::new();

    let mut event_dispatcher = EventDispatcher::new();
    event_dispatcher.add_handler(close_request_handler);
    event_dispatcher.add_handler(model_update_handler);
    event_dispatcher.add_handler(keyboard_handler);

    let mut frontend = Frontend2D::new();
    frontend.add_view(bird_view);

    let framework = Framework::new(model, frontend, event_dispatcher);
    framework.run(|model| model.keep_running());
}
