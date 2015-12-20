use client_state::frontend::{FrontendEvent};

pub struct InputState {
    keys: Vec<bool>
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            keys: vec![false; (GameButton::Max as usize)]
        }
    }

    pub fn key(&self, key: GameButton) -> bool {
        self.keys[key as usize]
    }

    pub fn set_key(&mut self, key: GameButton, state: bool) {
        self.keys[key as usize] = state;
    }

    pub fn update(&mut self, events: &[FrontendEvent]) {
        let iter = events.iter();
        for e in iter {
            match e {
                &FrontendEvent::Press(k) => self.set_key(k, true),
                &FrontendEvent::Release(k) => self.set_key(k, false)
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
    use client_state::input_state::{InputState, GameButton};
    use client_state::frontend::{FrontendEvent};

    #[test]
    fn key_returns_state_after_setting_key() {
        let mut input_state = InputState::new();
        input_state.set_key(GameButton::MoveCameraRight, true);

        assert!(input_state.key(GameButton::MoveCameraRight));
    }

    #[test]
    fn key_returns_state_after_update_receives_frontend_events() {
        let mut input_state = InputState::new();

        input_state.update(&vec!(FrontendEvent::Press(GameButton::MoveCameraRight)));
        assert!(input_state.key(GameButton::MoveCameraRight));

        input_state.update(&vec!(FrontendEvent::Release(GameButton::MoveCameraRight)));
        assert!(!input_state.key(GameButton::MoveCameraRight));
    }
}
