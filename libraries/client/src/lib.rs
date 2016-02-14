extern crate cgmath;
#[macro_use] extern crate enum_primitive;
extern crate time;

mod frame_timer;
mod input_state;
mod frontend;

pub use frame_timer::{UpdateDelta, FrameTimer};
pub use input_state::InputState;
pub use frontend::{FrontendEvent, GameButton};
