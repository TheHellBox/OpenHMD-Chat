use cobalt;
use player;
use render;

use bytevec::ByteDecodable;
use std::collections::HashMap;

use cobalt::{
    BinaryRateLimiter, Config, NoopPacketModifier, MessageKind, UdpSocket,
    Client, ClientEvent
};
use std::thread;

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
    pub fn connect(&mut self, addr: &'static str){
        self.client.connect(addr).expect("Failed to bind to socket.");
    }
    pub fn check(&mut self, players: &mut HashMap<u32, player::Player>, rendata: &mut render::RenderData) {
        while let Ok(event) = self.client.receive() {
            match event {
                ClientEvent::Message(message) => {
                    match message[0]{
                        0 => println!("{:?}", &message[1..message.len()]),
                        1 => println!("{:?}", &message[1..message.len()]),
                        2 => {
                            let player = player::Player::from_network(message[1..message.len()].to_vec());
                            let new_player = render::RenderObject{
                                mesh_name: "./assets/models/monkey.obj".to_string(),
                                tex_name: "none".to_string(),
                                position: player.position,
                                rotation: player.rotation
                            };
                            rendata.render_obj_buf.insert(player.id, new_player);
                            players.insert(player.id, player);
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
