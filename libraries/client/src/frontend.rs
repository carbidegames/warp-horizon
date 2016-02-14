use cgmath::Vector2;

enum_from_primitive! {
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum GameButton {
        MovePlayerRight,
        MovePlayerLeft,
        MovePlayerForward,
        MovePlayerBackward,
        RequestClose,
        Max
    }
}

pub enum FrontendEvent {
    CursorMove(Vector2<i32>),
    Press(GameButton),
    Release(GameButton),
    #[doc(hidden)]
    __DoNotMatch
}
