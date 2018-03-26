pub mod client_params;

use player;
use support;

use bytevec::{ByteEncodable, ByteDecodable};
use std::sync::mpsc;
use audio::AudioMsg;
use std::sync::mpsc::channel;

use cobalt::{
    BinaryRateLimiter, Config, NoopPacketModifier, MessageKind, UdpSocket,
    Client, ClientEvent
};

pub struct Network {
    client: Client<UdpSocket, BinaryRateLimiter, NoopPacketModifier>,
    //                   Data  Type  MessageKind
    pub tx: mpsc::Sender<(Vec<u8>, u8, MessageKind)>,
    pub rx: mpsc::Receiver<(Vec<u8>, u8, MessageKind)>,

    //                   Data  Type
    pub tx_back: mpsc::Sender<(Vec<u8>, u8)>,
    pub rx_back: mpsc::Receiver<(Vec<u8>, u8)>,

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
        use std::time::Duration;

        let mut config = Config::default();
        let (tx, rx) = channel::<(Vec<u8>, u8, MessageKind)>();
        let (tx_back, rx_back) = channel::<(Vec<u8>, u8)>();
        config.connection_closing_threshold = Duration::from_millis(5000);
        config.connection_drop_threshold = Duration::from_millis(5000);
        config.connection_init_threshold = Duration::from_millis(5000);
        config.send_rate = 1024;
        let client = Client::<UdpSocket, BinaryRateLimiter, NoopPacketModifier>::new(config);
        Network{
            client: client,
            tx: tx,
            rx: rx,
            tx_back: tx_back,
            rx_back: rx_back,
        }
    }
    pub fn connect(&mut self, addr: String){
        self.client.connect(addr).expect("Failed to bind to socket.");
    }
    pub fn check(&mut self, tx: &mpsc::Sender<player::Player>, tx_mobj: &mpsc::Sender<(Option<support::map_loader::MapObject>, Option<support::map_loader::Collider>)>
                , txsound: &mpsc::Sender<AudioMsg>, player: &player::Player){
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
                            let _ = tx.send(player);
                        },
                        //Sound
                        3 => {
                            let (rotx, roty, rotz, rotw) = player.rotation;
                            let (rotx, roty, rotz) = UnitQuaternion::from_quaternion(Quaternion::new(rotx, roty, rotz, rotw)).to_euler_angles();
                            let mut data = NetAudio::from_network(message[1..message.len()].to_vec());
                            let _ = txsound.send(AudioMsg{
                                data: data.data,
                                player_position: player.position,
                                player_rotation: (rotx, roty, rotz),
                                source_id: data.id,
                            });
                        },
                        //Map object
                        4 => {
                            let object = support::map_loader::MapObject::from_network(message[1..message.len()].to_vec());
                            let _ = tx_mobj.send((Some(object), None));
                        },
                        //Write file
                        5 => {
                            let msg = message[1..message.len()].to_vec();
                            self.tx_back.send((msg, 0));
                        },
                        6 => {
                            let collider = support::map_loader::Collider::from_network(message[1..message.len()].to_vec());
                            let _ = tx_mobj.send((None, Some(collider)));
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
        //FIXME: Poor code... Maybe...
        let mut msgs = vec![];
        {
            for x in self.rx.try_iter(){
                msgs.push(x);
            }
        }
        for x in msgs{
            let (data, type_d, msgk) = x;
            self.send(data, type_d, msgk);
        }
        self.client.send(true).is_ok();
    }

    pub fn send(&mut self, msg: Vec<u8>, type_d: u8, type_m: MessageKind){
        let mut msg = msg;
        msg.insert(0, type_d);
        let conn = self.client.connection().unwrap();
        conn.send(type_m, msg);
    }
}
