#[macro_use] extern crate glium;

mod frontend;
mod update;

use std::sync::mpsc;
use std::thread;

#[derive(Clone)]
pub struct GameState {
    t: f32
}

pub struct EngineToken;

pub fn run_client() {
    let (tx, rx) = mpsc::channel();

    let update = thread::spawn(move|| {
        update::update_runtime(tx);
    });

    let frontend = thread::spawn(move|| {
        frontend::frontend_runtime(rx);
    });

    update.join().unwrap();
    frontend.join().unwrap();
}
