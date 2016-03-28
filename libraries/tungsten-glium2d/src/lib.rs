extern crate dynamic;
#[macro_use] extern crate glium;
extern crate tungsten;

mod frontend;
mod runtime;

pub use frontend::{CloseRequestEvent, View2D, Frontend2D, RenderBatch};
