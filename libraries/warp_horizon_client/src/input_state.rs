use cgmath::Vector2;
use frontend::FrontendEvent;

/// A type representing the current state of player input in ways that the game can understand.
pub struct InputState {
    keys: Vec<bool>,
    mouse_position: Option<Vector2<i32>>,
    focused: bool,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            keys: vec![false; (GameButton::Max as usize)],
            mouse_position: None,
            focused: true,
        }
    }

    pub fn key(&self, key: GameButton) -> bool {
        self.keys[key as usize]
    }

    pub fn set_key(&mut self, key: GameButton, state: bool) {
        self.keys[key as usize] = state;
    }

    pub fn mouse_position(&self) -> Option<Vector2<i32>> {
        if self.focused {
            self.mouse_position
        } else {
            None
        }
    }

    pub fn set_mouse_position(&mut self, value: Vector2<i32>) {
        self.mouse_position = Some(value);
    }

    pub fn set_focused(&mut self, value: bool) {
        self.focused = value;
    }

    pub fn update(&mut self, events: &[FrontendEvent]) {
        for e in events.iter() {
            match e {
                &FrontendEvent::Press(k) => self.set_key(k, true),
                &FrontendEvent::Release(k) => self.set_key(k, false),
                &FrontendEvent::MouseMove(pos) => self.mouse_position = Some(pos),
                &FrontendEvent::Focus => self.focused = true,
                &FrontendEvent::Unfocus => self.focused = false,
                _ => {}
            }
        }
    }
}

enum_from_primitive! {
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum GameButton {
        MoveCameraRight,
        MoveCameraLeft,
        MoveCameraUp,
        MoveCameraDown,
        Max
    }
}

#[cfg(test)]
mod tests {
    use cgmath::Vector2;
    use input_state::{InputState, GameButton};
    use frontend::{FrontendEvent};

    #[test]
    fn key_returns_state_after_setting_key() {
        let mut input_state = InputState::new();
        input_state.set_key(GameButton::MoveCameraRight, true);

        assert!(input_state.key(GameButton::MoveCameraRight));
    }

    #[test]
    fn key_returns_state_after_update_receives_frontend_event() {
        let mut input_state = InputState::new();

        input_state.update(&vec!(FrontendEvent::Press(GameButton::MoveCameraRight)));
        assert!(input_state.key(GameButton::MoveCameraRight));

        input_state.update(&vec!(FrontendEvent::Release(GameButton::MoveCameraRight)));
        assert!(!input_state.key(GameButton::MoveCameraRight));
    }

    #[test]
    fn mouse_position_returns_position_after_update_receives_mouse_move() {
        let mut input_state = InputState::new();
        input_state.update(&vec!(FrontendEvent::MouseMove(Vector2::new(50, 23))));
        assert_eq!(input_state.mouse_position(), Some(Vector2::new(50, 23)));
    }

    #[test]
    fn mouse_position_returns_none_after_update_receives_unfocus() {
        let mut input_state = InputState::new();
        let mouse_move = vec!(FrontendEvent::MouseMove(Vector2::new(50, 23)));
        let unfocus = vec!(FrontendEvent::Unfocus);

        // Initial should be none
        assert!(input_state.mouse_position().is_none());

        // After a move and an unfocus, it should have none
        input_state.update(&mouse_move);
        input_state.update(&unfocus);
        assert!(input_state.mouse_position().is_none());

        // After another mouse update it still should have none
        input_state.update(&mouse_move);
        assert!(input_state.mouse_position().is_none());
    }
}
