use std::{thread, time};
use std::sync::mpsc::{Sender, Receiver};
use bincode::{serialize, deserialize};
use std::sync::mpsc::channel;
use nalgebra::{Translation3, Point3, UnitQuaternion};
use cobalt::{
    BinaryRateLimiter, Config, NoopPacketModifier, MessageKind, UdpSocket,
    Server, ServerEvent, ConnectionID
};

#[derive(Serialize, Deserialize)]
pub enum NetworkEvent{
    SendMsg(Vec<u8>),
    SendAudio(Vec<u8>),
    SendPosition(Point3<f32>),
    SendRotation(UnitQuaternion<f32>),
}

pub enum NetworkCommand{
    SendGameObjects(u32)
}

#[derive(Serialize, Deserialize)]
pub enum MessageType{
    EncodedAudio(Vec<u8>, u32),
    PlayerConnected(u32),
    PlayerDisconnected(u32),
    CreateGameObject(String),
    GameObjectChangedPosition(String, Point3<f32>),
    GameObjectChangedModel(String, String),
    GameObjectChangedRotation(String, UnitQuaternion<f32>),
    AudioEvent(Vec<u8>),
    ServerInfo(Vec<u8>),
    LuaScript(String),
}

pub enum MsgDst{
    Boardcast(),
    Id(u32)
}

#[derive(Serialize, Deserialize)]
pub struct ServerInfo{
    players: Vec<u32>,
    send_rate: u32,
    audio_quality: u32,
}

pub struct Network {
    pub server: Server<UdpSocket,BinaryRateLimiter,NoopPacketModifier>,
    pub server_info: ServerInfo,
    pub tx_in: Sender<(MessageKind, MessageType, MsgDst)>,
    rx_in: Receiver<(MessageKind, MessageType, MsgDst)>,
    tx_out: Sender<NetworkCommand>,
}

impl Network {
    pub fn new() -> (Network, Receiver<NetworkCommand>){
        use std::time::Duration;

        let mut config = Config::default();
        config.connection_closing_threshold = Duration::from_millis(15000);
        config.connection_drop_threshold = Duration::from_millis(5000);
        config.connection_init_threshold = Duration::from_millis(15000);
        config.send_rate = 1024;
        let server = Server::<UdpSocket, BinaryRateLimiter, NoopPacketModifier>::new(config);
        let server_info = ServerInfo{
            players: vec![],
            send_rate: 1024,
            audio_quality: 16000
        };
        let (tx_in, rx_in) = channel::<(MessageKind, MessageType, MsgDst)>();
        let (tx_out, rx_out) = channel::<NetworkCommand>();
        (Network{
            server,
            server_info,
            tx_in,
            rx_in,
            tx_out
        },
        rx_out)
    }

    pub fn listen(&mut self, ip: &'static str){
        self.server.listen(ip).expect("Failed to bind to socket.");
    }

    pub fn init(&mut self){
        loop{
            for x in self.rx_in.try_iter(){
                for (_, conn) in self.server.connections() {
                    conn.send(x.0, serialize(&x.1).unwrap());
                }
            }
            while let Ok(event) = self.server.accept_receive() {
                match event{
                    ServerEvent::Message(id, message) => {
                        let ConnectionID(id_u32) = id;
                        let message = deserialize(&message).unwrap();
                        match message{
                            NetworkEvent::SendAudio(x) => {
                                for (_, conn) in self.server.connections() {
                                    if conn.id() != id{
                                        let audio = MessageType::EncodedAudio(x.clone(), id_u32);
                                        conn.send(MessageKind::Instant, serialize(&audio).unwrap());
                                    }
                                }
                            },
                            NetworkEvent::SendPosition(position) => {
                                for (_, conn) in self.server.connections() {
                                    if conn.id() != id{
                                        let msg = MessageType::GameObjectChangedPosition(format!{"player{}", id_u32}, position);
                                        conn.send(MessageKind::Instant, serialize(&msg).unwrap());
                                    }
                                }
                            },
                            NetworkEvent::SendRotation(rotation) => {
                                for (_, conn) in self.server.connections() {
                                    if conn.id() != id{
                                        let msg = MessageType::GameObjectChangedRotation(format!{"player{}", id_u32}, rotation);
                                        conn.send(MessageKind::Instant, serialize(&msg).unwrap());
                                    }
                                }
                            },
                            _ => {}
                        }
                    },
                    ServerEvent::Connection(id) => {

                        let ConnectionID(id_u32) = id;
                        self.server_info.players.push(id_u32);
                        println!("player{} has been connected", id_u32);
                        for (_, conn) in self.server.connections() {
                            if conn.id() != id{
                                let ConnectionID(player_id) = id;
                                let player_connected = MessageType::PlayerConnected(id_u32);
                                conn.send(MessageKind::Reliable, serialize(&player_connected).unwrap());
                                self.server_info.players.push(player_id);
                            }
                            else{
                                let server_info_raw = serialize(&self.server_info).unwrap();
                                let server_info = serialize(&MessageType::ServerInfo(server_info_raw)).unwrap();
                                self.tx_out.send(NetworkCommand::SendGameObjects(id_u32));
                                conn.send(MessageKind::Reliable, server_info);
                            }
                        }
                    },
                    ServerEvent::ConnectionLost(id, _) => {
                        let ConnectionID(player_id) = id;
                        self.server_info.players.retain(|i| *i == player_id);
                        println!("player{} has been disconected", player_id);
                        for (_, conn) in self.server.connections() {
                            if conn.id() != id{
                                let player_disconnected = MessageType::PlayerDisconnected(player_id);
                                conn.send(MessageKind::Reliable, serialize(&player_disconnected).unwrap());
                            }
                        }
                    },
                    _ => {}
                }
            }
            self.server.send(true).is_ok();
            thread::sleep(time::Duration::from_millis(1));
        }
    }
}
