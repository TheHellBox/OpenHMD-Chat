use alto;
use opus;
use alto::*;
use player;

use std::sync::mpsc;
use std::collections::HashMap;
use std::thread;

pub struct AudioWrapper{
    pub alto: Alto
}

pub struct AudioStaticSource{
    pub source: StreamingSource
}

pub struct AudioMsg {
    pub data: Vec<u8>,
    pub player_position: (f32,f32,f32),
    pub player_rotation: (f32,f32,f32),
    pub source_id: u32
}

impl AudioWrapper{
    pub fn new() -> Result<AudioWrapper, &'static str>{
        let alto = match Alto::load_default(){
            Ok(x) => x,
            _ => {
                return Err("Failed to init OpenAL");
            }
        };

        Ok(AudioWrapper{
            alto: alto,
        })
    }
    pub fn create_output(&self) -> Option<alto::OutputDevice>{
        let output = match self.alto.open(None){
            Ok(x) => x,
            _ => {
                return None
            }
        };
        Some(output)
    }

    pub fn create_capture(&self) -> Option<alto::Capture<Mono<i16>>>{
        let capture : alto::Capture<Mono<i16>> = match self.alto.open_capture(None, 16000, 2048){
            Ok(x) => x,
            _ => {
                return None;
            }
        };
        Some(capture)
    }

    pub fn get_context(&self, dev: alto::OutputDevice) -> Option<alto::Context>{
        let ctx = match dev.new_context(None){
            Ok(x) => x,
            _ => {
                return None
            }
        };
        Some(ctx)
    }
    pub fn start_threads(&self, tx_netsound_in: mpsc::Sender<AudioMsg>, rx_netsound_out: mpsc::Receiver<AudioMsg>, rx_players: mpsc::Receiver<HashMap<u32, player::Player>> ){
        let capture = self.create_capture();
        let output = self.create_output();
        if capture.is_some(){
            thread::spawn(move || {
                start_audio_capture(tx_netsound_in, capture.unwrap());
            });
        }
        else{
            println!("Failed to init audio capture");
        }
        if output.is_some(){
            let context = self.get_context(output.unwrap());
            thread::spawn(move || {
                start_audio_playback(rx_netsound_out, rx_players, context.unwrap());
            });
        }
        else{
            println!("Failed to init audio output");
        }
    }
}

impl AudioStaticSource{

}

pub fn start_audio_capture(tx: mpsc::Sender<AudioMsg>, mut capture: alto::Capture<Mono<i16>>){
    let mut opus_encoder = opus::Encoder::new(16000, opus::Channels::Mono, opus::Application::Voip).unwrap();
    capture.start();
    loop{
        let mut buffer: Vec<alto::Mono<i16>> = vec![alto::Mono::<i16> { center : 0 }; 320 as usize];
        let mut samples_len = capture.samples_len();
        if samples_len >= buffer.len() as i32{
            capture.capture_samples(&mut buffer).unwrap();
            let encode_buf = buffer.iter().map(|&x| x.center).collect::<Vec<_>>();
            let encoded = opus_encoder.encode_vec(&encode_buf, 16000).unwrap();
            tx.send(AudioMsg{
                data: encoded,
                player_position: (0.0,0.0,0.0),
                player_rotation: (0.0,0.0,0.0),
                source_id: 0,
            });
            thread::sleep_ms(16);
        }
    }
}

pub fn start_audio_playback(rx: mpsc::Receiver<AudioMsg>, rx_players: mpsc::Receiver<HashMap<u32, player::Player>>, context: alto::Context){
    let mut opus_decoder = opus::Decoder::new(16000, opus::Channels::Mono).unwrap();
    let mut sources = HashMap::new();
    loop{
        let playerslist = rx_players.try_iter();
        for x in playerslist{
            for (id, player) in x{
                let mut src = context.new_streaming_source();
                if src.is_ok(){
                    let src = src.unwrap();
                    sources.entry(id).or_insert(src);
                    let src = sources.get_mut(&id).unwrap();
                    let (posx, posy, posz) = player.position;
                    src.set_position([-posx, posy, -posz]);
                }
            }
        }
        let data = rx.try_iter().last();
        if data.is_some(){
            let data = data.unwrap();
            let src = sources.get_mut(&data.source_id);
            if src.is_some(){
                let src = src.unwrap();
                let mut buffers_avail = src.buffers_processed();
                while buffers_avail > 0 {
                    let buf = src.unqueue_buffer();
                    if buf.is_ok(){
                        buf.unwrap();
                    }
                    else{
                        break;
                    }
                }
                let (posx, posy, posz) = data.player_position;
                let (rotx, roty, rotz) = data.player_rotation;
                let data = data.data;
                context.set_position([-posx, -posy, -posz]);
                context.set_orientation(([1.0,1.0,1.0], [rotx, roty, rotz])).unwrap();

                let mut decode = vec![0i16; 320];
                opus_decoder.decode(&data, &mut decode, false).unwrap();
                let decode = decode.into_iter().map(|x| alto::Mono::<i16> { center : x }).collect::<Vec<alto::Mono::<i16>>>();
                let buf = context.new_buffer::<alto::Mono<i16>,_>(&decode, 16000).unwrap();
                src.queue_buffer(buf);
                if src.state() != alto::SourceState::Playing {
                    src.play();
                }
            }
        }
    }
}
