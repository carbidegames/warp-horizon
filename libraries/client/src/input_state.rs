use cgmath::Vector2;
use ::{FrontendEvent, GameButton};

/// A type representing the current state of player input in ways that the game can understand.
pub struct InputState {
    keys: Vec<bool>,
    mouse_position: Option<Vector2<i32>>,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            keys: vec![false; (GameButton::Max as usize)],
            mouse_position: None,
        }
    }

    pub fn key(&self, key: GameButton) -> bool {
        self.keys[key as usize]
    }

    pub fn set_key(&mut self, key: GameButton, state: bool) {
        self.keys[key as usize] = state;
    }

    pub fn mouse_position(&self) -> Option<Vector2<i32>> {
        self.mouse_position
    }

    pub fn set_mouse_position(&mut self, value: Vector2<i32>) {
        self.mouse_position = Some(value);
    }

    pub fn update(&mut self, events: &[FrontendEvent]) {
        for e in events.iter() {
            match e {
                &FrontendEvent::Press(k) => self.set_key(k, true),
                &FrontendEvent::Release(k) => self.set_key(k, false),
                &FrontendEvent::CursorMove(pos) => self.mouse_position = Some(pos),
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use cgmath::Vector2;
    use ::{FrontendEvent, InputState, GameButton};

    #[test]
    fn key_returns_state_after_setting_key() {
        let mut input_state = InputState::new();
        input_state.set_key(GameButton::MovePlayerRight, true);

        assert!(input_state.key(GameButton::MovePlayerRight));
    }

    #[test]
    fn key_returns_state_after_update_receives_frontend_event() {
        let mut input_state = InputState::new();

        input_state.update(&vec!(FrontendEvent::Press(GameButton::MovePlayerRight)));
        assert!(input_state.key(GameButton::MovePlayerRight));

        input_state.update(&vec!(FrontendEvent::Release(GameButton::MovePlayerRight)));
        assert!(!input_state.key(GameButton::MovePlayerRight));
    }

    #[test]
    fn mouse_position_returns_position_after_update_receives_mouse_move() {
        let mut input_state = InputState::new();
        input_state.update(&vec!(FrontendEvent::CursorMove(Vector2::new(50, 23))));
        assert_eq!(input_state.mouse_position(), Some(Vector2::new(50, 23)));
    }
}
