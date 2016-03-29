extern crate cgmath;
#[macro_use] extern crate glium;
extern crate tungsten;

mod frontend;
mod runtime;

pub use frontend::{CloseRequestEvent, View2D, Frontend2D, RenderBatch, KeyboardInputEvent};

// Re-export as utility
pub use glium::glutin::VirtualKeyCode as Key;
pub use glium::glutin::ElementState as KeyState;
