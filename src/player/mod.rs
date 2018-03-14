use render;

use bytevec::ByteEncodable;
use bytevec::ByteDecodable;
#[derive(PartialEq, Debug, Default, Clone)]
pub struct Player {
    pub id: u32,
    pub position: (f32, f32, f32),
    pub rotation: (f32, f32, f32, f32),
    pub model: String,
    pub name: String
}
pub struct LocalPlayer {
    pub position: (f32, f32, f32),
    pub rotation: (f32, f32, f32, f32),

    pub player_speed_f: f32,
    pub player_speed_lr: f32,

    pub ghost_position: (f32, f32, f32),
    pub player_moving: bool
}

impl Player {
    pub fn to_network(&self) -> Vec<u8>{
        self.encode::<u8>().unwrap()
    }
    pub fn from_network(message: Vec<u8>) -> Player{
        Player::decode::<u8>(&message).unwrap()
    }
}
bytevec_impls! {
    impl Player {
        id: u32,
        position: (f32, f32, f32),
        rotation: (f32, f32, f32, f32),
        model: String,
        name: String
    }
}
