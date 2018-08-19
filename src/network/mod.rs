use opus;
use std::{thread, time};
use audio::AudioEvent;
use std::sync::mpsc::{channel, Sender, Receiver};
use bincode::{deserialize, serialize};

#[derive(Serialize, Deserialize)]
pub enum NetworkEvent{
    SendMsg(Vec<u8>),
    SendAudio(Vec<u8>),
}

#[derive(Serialize, Deserialize)]
enum MessageType{
    EncodedAudio(Vec<u8>, u32),
    PlayerConnected(u32),
    AudioEvent(Vec<u8>),
    ServerInfo(Vec<u8>),
}

#[derive(Serialize, Deserialize)]
pub struct ServerInfo{
    players: Vec<u32>,
    send_rate: u32,
    audio_quality: u32,
}

use cobalt::{
    BinaryRateLimiter, Config, NoopPacketModifier, MessageKind, UdpSocket,
    Client, ClientEvent
};

pub struct Network{
    client: Client<UdpSocket, BinaryRateLimiter, NoopPacketModifier>,
    pub tx: Sender<NetworkEvent>,
    rx: Receiver<NetworkEvent>
}

impl Network {
    pub fn new() -> Network{
        use std::time::Duration;

        let mut config = Config::default();
        let (tx, rx) = channel::<NetworkEvent>();

        config.connection_closing_threshold = Duration::from_millis(15000);
        config.connection_drop_threshold = Duration::from_millis(5000);
        config.connection_init_threshold = Duration::from_millis(15000);
        config.send_rate = 1024;
        let client = Client::<UdpSocket, BinaryRateLimiter, NoopPacketModifier>::new(config);
        Network{
            client,
            tx,
            rx
        }
    }

    pub fn connect(&mut self, addr: String){
        self.client.connect(addr).expect("Failed to bind to socket.");
    }

    pub fn init(&mut self, tx_audio: Sender<AudioEvent>){
        let mut opus_decoder = opus::Decoder::new(16000, opus::Channels::Mono).unwrap();
        loop{
            for x in self.rx.try_iter(){
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
                            },
                            MessageType::ServerInfo(data) => {
                                println!("Connected!");
                                let server_info: ServerInfo = deserialize(&data).unwrap();
                                for x in server_info.players{
                                    println!("{}", x);
                                    let _ = tx_audio.send(AudioEvent::AddSource(format!("player{}", x).to_string()));
                                }
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
