use cgmath::{Vector3, Quaternion};

pub struct Player {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
}

pub struct ClientState {
    pub player: Player
}
