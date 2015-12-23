use cgmath::Vector2;
use input_state::GameButton;

pub enum FrontendEvent {
    Press(GameButton),
    Release(GameButton),
    MouseMove(Vector2<i32>)
}
