use player;
use bytevec::{ByteEncodable, ByteDecodable};
use std::sync::mpsc;
use audio::AudioMsg;

use cobalt::{
    BinaryRateLimiter, Config, NoopPacketModifier, MessageKind, UdpSocket,
    Client, ClientEvent
};

pub struct Network {
    client: Client<UdpSocket, BinaryRateLimiter, NoopPacketModifier>
}

#[derive(PartialEq, Debug, Default, Clone)]
pub struct NetAudio {
    pub data: Vec<u8>,
    pub id: u32
}
bytevec_impls! {
    impl NetAudio {
        data: Vec<u8>,
        id: u32
    }
}
impl NetAudio {
    pub fn to_network(&self) -> Vec<u8>{
        self.encode::<u8>().unwrap()
    }
    pub fn from_network(message: Vec<u8>) -> NetAudio{
        NetAudio::decode::<u8>(&message).unwrap()
    }
}

impl Network {
    pub fn new() -> Network{
        let mut client = Client::<UdpSocket, BinaryRateLimiter, NoopPacketModifier>::new(Config::default());
        Network{
            client: client
        }
    }
    pub fn connect(&mut self, addr: String){
        self.client.connect(addr).expect("Failed to bind to socket.");
    }
    pub fn check(&mut self, tx: &mpsc::Sender<player::Player>, txsound: &mpsc::Sender<AudioMsg>, player: &player::Player) {
        use nalgebra::geometry::UnitQuaternion;
        use nalgebra::geometry::Quaternion;

        while let Ok(event) = self.client.receive() {
            match event {
                ClientEvent::Message(message) => {
                    match message[0]{
                        0 => println!("{:?}", &message[1..message.len()]),
                        1 => println!("{:?}", &message[1..message.len()]),
                        2 => {
                            let player = player::Player::from_network(message[1..message.len()].to_vec());
                            tx.send(player);
                        },
                        3 => {
                            let (rotx, roty, rotz, rotw) = player.rotation;
                            let (rotx, roty, rotz) = UnitQuaternion::from_quaternion(Quaternion::new(rotx, roty, rotz, rotw)).to_euler_angles();
                            let mut data = NetAudio::from_network(message[1..message.len()].to_vec());
                            txsound.send(AudioMsg{
                                data: data.data,
                                player_position: player.position,
                                player_rotation: (rotx, roty, rotz),
                                source_id: data.id,
                            });
                        },
                        _ => {}
                    }
                },
                _ => println!("{:?}", event)
            }
        }
        self.client.send(true).is_ok();
    }

    pub fn send(&mut self, msg: Vec<u8>, type_d: u8){
        let mut msg = msg;
        msg.insert(0, type_d);
        let conn = self.client.connection().unwrap();
        conn.send(MessageKind::Instant, msg);
    }
}
