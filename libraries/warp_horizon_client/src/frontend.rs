use input_state::GameButton;

pub enum FrontendEvent {
    Press(GameButton),
    Release(GameButton),
}