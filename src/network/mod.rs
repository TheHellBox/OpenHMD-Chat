use opus;
use scripting_engine::{LUA_CHANNL_OUT, LuaEvent};
use std::{thread, time};
use audio::AudioEvent;
use std::sync::{Mutex};
use nalgebra::{Point3, UnitQuaternion};
use std::sync::mpsc::{channel, Sender, Receiver};
use bincode::{deserialize, serialize};

lazy_static! {
    pub static ref CONNECTON_ID: Mutex<u32> = Mutex::new(0);
}

#[derive(Serialize, Deserialize)]
pub enum NetworkEvent{
    SendMsg(Vec<u8>),
    SendAudio(Vec<u8>),
    SendPosition(Point3<f32>),
    SendRotation(UnitQuaternion<f32>),
}

#[derive(Serialize, Deserialize)]
pub enum NetworkCommand{
    CreatePlayerGameobject(u32),
    CreateGameobject(String),
    RemovePlayerGameobject(u32),
    ChangeGameObjectPosition(String, Point3<f32>),
    ChangeGameObjectScale(String, (f32, f32, f32)),
    ChangeGameObjectRotation(String, UnitQuaternion<f32>),
    ChangeGameObjectModel(String, String),
    SendPlayerInfo(),
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
    GameObjectChangedScale(String, (f32, f32, f32)),
    AudioEvent(Vec<u8>),
    ServerInfo(Vec<u8>),
    LuaScript(String),
}

#[derive(Serialize, Deserialize)]
pub struct ServerInfo{
    players: Vec<u32>,
    send_rate: u32,
    audio_quality: u32,
}

use cobalt::{
    BinaryRateLimiter, Config, NoopPacketModifier, MessageKind, UdpSocket,
    Client, ClientEvent, ConnectionID
};

pub struct Network{
    client: Client<UdpSocket, BinaryRateLimiter, NoopPacketModifier>,
    pub tx_in: Sender<NetworkEvent>,
    rx_in: Receiver<NetworkEvent>,
    tx_out: Sender<NetworkCommand>,
}

impl Network {
    pub fn new() -> (Network, Receiver<NetworkCommand>){
        use std::time::Duration;

        let mut config = Config::default();
        let (tx_in, rx_in) = channel::<NetworkEvent>();
        let (tx_out, rx_out) = channel::<NetworkCommand>();

        config.connection_closing_threshold = Duration::from_millis(15000);
        config.connection_drop_threshold = Duration::from_millis(5000);
        config.connection_init_threshold = Duration::from_millis(15000);
        config.send_rate = 1024;
        let client = Client::<UdpSocket, BinaryRateLimiter, NoopPacketModifier>::new(config);
        (Network{
            client,
            tx_in,
            rx_in,
            tx_out
        },
        rx_out)
    }

    pub fn connect(&mut self, addr: String){
        self.client.connect(addr).expect("Failed to bind to socket.");
        if let Ok(conn) = self.client.connection() {
            let ConnectionID(id_u32) = conn.id();
            *CONNECTON_ID.lock().unwrap() = id_u32;
        }
    }

    pub fn init(&mut self, tx_audio: Sender<AudioEvent>){
        let mut opus_decoder = opus::Decoder::new(16000, opus::Channels::Mono).unwrap();
        loop{
            for x in self.rx_in.try_iter(){
                if let Ok(conn) = self.client.connection() {
                    conn.send(MessageKind::Instant, serialize(&x).unwrap());
                }
            }

            while let Ok(event) = self.client.receive() {
                match event {
                    ClientEvent::Message(message) => {
                        let decoded: MessageType = deserialize(&message).unwrap();
                        match decoded{
                            MessageType::EncodedAudio(data, id) => {
                                let mut decode = vec![0i16; 1280];
                                opus_decoder.decode(&data, &mut decode, false).unwrap();
                                let _ = tx_audio.send(AudioEvent::Play(decode, 16000, format!("player{}", id)));
                            },
                            MessageType::PlayerConnected(id) => {
                                let _ = tx_audio.send(AudioEvent::AddSource(format!("player{}", id).to_string()));
                                //let _ = self.tx_out.send(NetworkCommand::CreatePlayerGameobject(id));
                                let _ = self.tx_out.send(NetworkCommand::SendPlayerInfo());
                            },
                            MessageType::PlayerDisconnected(id) => {
                                let _ = tx_audio.send(AudioEvent::RemoveSource(format!("player{}", id).to_string()));
                                let _ = self.tx_out.send(NetworkCommand::RemovePlayerGameobject(id));
                            },
                            MessageType::CreateGameObject(name) => {
                                let _ = self.tx_out.send(NetworkCommand::CreateGameobject(name));
                            },
                            MessageType::GameObjectChangedPosition(name, position) => {
                                let _ = self.tx_out.send(NetworkCommand::ChangeGameObjectPosition(name, position));
                            },
                            MessageType::GameObjectChangedRotation(name, rotation) => {
                                let _ = self.tx_out.send(NetworkCommand::ChangeGameObjectRotation(name, rotation));
                            },
                            MessageType::GameObjectChangedModel(name, model) => {
                                let _ = self.tx_out.send(NetworkCommand::ChangeGameObjectModel(name, model));
                            },
                            MessageType::GameObjectChangedScale(name, scale) => {
                                println!("changed scale");
                                let _ = self.tx_out.send(NetworkCommand::ChangeGameObjectScale(name, scale));
                            },
                            MessageType::LuaScript(script) => {
                                println!("lua script");
                                let channels = LUA_CHANNL_OUT.0.lock().unwrap();
                                let _ = channels.send(LuaEvent::RunLua(script));
                            },
                            MessageType::ServerInfo(data) => {
                                println!("Connected!");
                                let server_info: ServerInfo = deserialize(&data).unwrap();
                                for x in server_info.players{
                                    let _ = tx_audio.send(AudioEvent::AddSource(format!("player{}", x).to_string()));
                                    let _ = self.tx_out.send(NetworkCommand::CreatePlayerGameobject(x));
                                }
                                let _ = self.tx_out.send(NetworkCommand::SendPlayerInfo());
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
            self.client.send(true).is_ok();
            thread::sleep(time::Duration::from_millis(1));
        }
    }
}
