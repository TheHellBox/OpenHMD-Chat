
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

    pub camera_position: (f32, f32, f32),
    pub camera_rotation: (f32, f32, f32, f32),

    pub player_moving: bool
}

impl LocalPlayer {
    pub fn new(pos: (f32,f32,f32)) -> LocalPlayer{
        LocalPlayer{
            position: pos,
            rotation: (0.0,0.0,1.0,0.0),

            player_speed_f: 0.0,
            player_speed_lr: 0.0,

            camera_position: pos,
            camera_rotation: (0.0,0.0,1.0,0.0),
            player_moving: false
        }
    }
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
