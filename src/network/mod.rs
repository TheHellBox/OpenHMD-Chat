use player;

use std::sync::mpsc;
use audio::AudioMsg;

use cobalt::{
    BinaryRateLimiter, Config, NoopPacketModifier, MessageKind, UdpSocket,
    Client, ClientEvent
};

pub struct Network {
    client: Client<UdpSocket, BinaryRateLimiter, NoopPacketModifier>
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
                            txsound.send(AudioMsg{
                                data: message[1..message.len()].to_vec(),
                                player_position: player.position,
                                source_id: 0,
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
