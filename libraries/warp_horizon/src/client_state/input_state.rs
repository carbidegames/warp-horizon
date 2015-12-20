pub struct InputState {
    keys: Vec<bool>
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            keys: vec![false; (GameKey::Max as usize)]
        }
    }

    pub fn key(&self, key: GameKey) -> bool {
        self.keys[key as usize]
    }

    pub fn set_key(&mut self, key: GameKey, state: bool) {
        self.keys[key as usize] = state;
    }
}

enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    pub enum GameKey {
        MoveCameraRight,
        MoveCameraLeft,
        MoveCameraUp,
        MoveCameraDown,
        Max
    }
}

#[cfg(test)]
mod tests {
    use client_state::input_state::{InputState, GameKey};

    #[test]
    fn key_returns_state_after_setting_key() {
        let mut input_state = InputState::new();
        input_state.set_key(GameKey::MoveCameraRight, true);

        assert!(input_state.key(GameKey::MoveCameraRight));
    }
}
