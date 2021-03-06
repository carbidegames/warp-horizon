extern crate tungsten;
extern crate tungsten_glium2d;

use tungsten::{Framework, EventDispatcher, UpdateEvent};
use tungsten_glium2d::{Frontend2D, CloseRequestEvent, FrameRenderInfo, KeyboardInputEvent, Key, KeyState, RenderTarget, Rectangle, View2D, TextureId};

enum BirdState {
    Alive,
    Dead(f32)
}

struct GameModel {
    should_close: bool,
    bird_height: f32,
    bird_velocity: f32,
    bird_state: BirdState,
    camera_distance: f32,
}

impl GameModel {
    fn new() -> Self {
        GameModel {
            should_close: false,
            bird_height: 64.0,
            bird_velocity: 0.0,
            bird_state: BirdState::Alive,
            camera_distance: 0.0,
        }
    }

    fn update(&mut self, delta: f32) {
        // Advance the game state
        self.camera_distance += 32.0 * 6.0 * delta;

        // Different paths for the state of the bird
        match self.bird_state {
            BirdState::Alive => {
                // Make the bird fall
                self.bird_velocity -= 32.0 * 24.0 * delta;
                self.bird_height += self.bird_velocity * delta;

                // If the bird falls below this, it's dead now, you killed it, you monster
                if self.bird_height < -300.0 {
                    self.bird_state = BirdState::Dead(self.camera_distance);
                }
            },
            BirdState::Dead(_distance) => {
                // It's dead, nothing happens
            },
        }
    }

    fn launch_bird(&mut self) {
        self.bird_velocity = 32.0 * 18.0;
    }

    fn reset_game(&mut self) {
        self.bird_height = 64.0;
        self.bird_velocity = 0.0;
        self.bird_state = BirdState::Alive;
        self.camera_distance = 0.0;
    }

    fn close(&mut self) {
        self.should_close = true;
    }

    fn keep_running(&self) -> bool {
        !self.should_close
    }
}

fn close_request_handler(model: &mut GameModel, _event: &CloseRequestEvent) {
    model.close();
}

fn update_handler(model: &mut GameModel, event: &UpdateEvent) {
    model.update(event.delta);
}

fn keyboard_handler(model: &mut GameModel, event: &KeyboardInputEvent) {
    if event.state == KeyState::Pressed {
        match event.key {
            Key::W => model.launch_bird(),
            Key::R => model.reset_game(),
            Key::Escape => model.close(),
            _ => ()
        }
    }
}

struct View {
    bird: TextureId,
    ground: TextureId,
    youdied: TextureId,
}

impl View {
    fn new(frontend: &mut Frontend2D<GameModel>) -> Self {
        // Load in textures
        let bird = frontend.load_texture("./assets/bird.png");
        let ground = frontend.load_texture("./assets/grass.png");
        let youdied = frontend.load_texture("./assets/youdied.png");

        View {
            bird: bird,
            ground: ground,
            youdied: youdied,
        }
    }

    fn render_world(&self, model: &GameModel, info: &mut FrameRenderInfo) {
        let camera = info.game_camera([model.camera_distance, 0.0]);
        let batch = camera.batch();

        // Draw the terrain
        for i in -10..100 {
            let rect = Rectangle {
                position: [i as f32 * 64.0, -720.0/2.0],
                size: [64.0, 64.0],
                texture: self.ground,
            };
            batch.rectangle(rect);
        }

        // Draw the bird
        let dist = {
            if let BirdState::Dead(dist) = model.bird_state { dist }
            else { model.camera_distance }
        };
        let rect = Rectangle {
            position: [dist, model.bird_height],
            size: [64.0, 64.0],
            texture: self.bird,
        };
        batch.rectangle(rect);
    }

    fn render_ui(&self, model: &GameModel, info: &mut FrameRenderInfo) {
        let camera = info.game_camera([0.0, 0.0]);
        let batch = camera.batch();

        if let BirdState::Dead(_) = model.bird_state {
            let rect = Rectangle {
                position: [0.0, 0.0],
                size: [256.0, 256.0],
                texture: self.youdied,
            };
            batch.rectangle(rect);
        }
    }
}

impl View2D<GameModel> for View {
    fn render(&mut self, model: &GameModel, info: &mut FrameRenderInfo) {
        self.render_world(model, info);
        self.render_ui(model, info);
    }
}

fn main() {
    let model = GameModel::new();

    let mut event_dispatcher = EventDispatcher::new();
    event_dispatcher.add_handler(close_request_handler);
    event_dispatcher.add_handler(update_handler);
    event_dispatcher.add_handler(keyboard_handler);

    let mut frontend = Frontend2D::new();
    let view = View::new(&mut frontend);
    frontend.set_view(view);

    let framework = Framework::new(model, frontend, event_dispatcher);
    framework.run(|model| model.keep_running());
}
