use opus;
use alto::*;
use std::{thread, time};
use network::NetworkEvent;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Sender};

#[derive(Serialize, Deserialize)]
pub enum AudioEvent{
    Play(Vec<i16>, i32, String),
    AddSource(String),
    RemoveSource(String)
}

pub struct AudioWrapper{
    pub alto: Alto
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

    pub fn create_output(&self) -> Option<OutputDevice>{
        let output = match self.alto.open(None){
            Ok(x) => x,
            _ => {
                return None
            }
        };
        Some(output)
    }

    pub fn get_context(&self, dev: OutputDevice) -> Option<Context>{
        let ctx = match dev.new_context(None){
            Ok(x) => x,
            _ => {
                return None
            }
        };
        Some(ctx)
    }

    pub fn create_capture(&self, sample_rate: i32, frames: i32) -> Option<Capture<Mono<i16>>>{
        let capture : Capture<Mono<i16>> = match self.alto.open_capture(None, sample_rate as u32, frames){
            Ok(x) => x,
            _ => {
                return None;
            }
        };
        Some(capture)
    }
    pub fn init(&self, sample_rate: i32, frames: i32, network_tx: Sender<NetworkEvent>) -> Sender<AudioEvent>{
        let (tx_audio, rx_audio) = channel::<AudioEvent>();

        let output = self.create_output().unwrap();
        let context = self.get_context(output).unwrap();

        let mut capture = self.create_capture(sample_rate, frames).unwrap();

        let mut sources: HashMap<String, StreamingSource> = HashMap::with_capacity(512);

        let src = context.new_streaming_source().unwrap();
        sources.insert("default".to_string(), src);

        thread::spawn(move || {
            let mut opus_encoder = opus::Encoder::new(sample_rate as u32, opus::Channels::Mono, opus::Application::Voip).unwrap();
            capture.start();
            loop{
                let mut samples_len = capture.samples_len();
                let mut buffer_i16: Vec<i16> = vec![0; 1280 as usize];
                while samples_len < buffer_i16.len() as i32 {
                    samples_len = capture.samples_len();
                    thread::sleep(time::Duration::from_millis(1));
                }
                capture.capture_samples(&mut buffer_i16).unwrap();
                let encoded = opus_encoder.encode_vec(&buffer_i16, sample_rate as usize).unwrap();
                let _ = network_tx.send(NetworkEvent::SendAudio(encoded));
                thread::sleep(time::Duration::from_millis(1));
            }
        });

        thread::spawn(move || {
            loop{
                let data = rx_audio.try_iter();
                for buf in data{
                    match buf{
                        AudioEvent::Play(data, rate, source_name) => {
                            let buf = context.new_buffer::<Mono<i16>,_>(data, rate).unwrap();
                            let src = sources.get_mut(&source_name);
                            if let Some(src) = src{
                                let _ = src.unqueue_buffer();
                                let _ = src.queue_buffer(buf);
                            }
                            else{
                                println!("[ERROR] Unknow audio source");
                            }
                        },
                        AudioEvent::AddSource(name) => {
                            let src = context.new_streaming_source().unwrap();
                            sources.insert(name, src);
                        },
                        AudioEvent::RemoveSource(name) => {
                            sources.remove(&name);
                        },
                        _ => {}
                    }
                }
                for (_, src) in &mut sources{
                    if src.state() != SourceState::Playing {
                        src.play();
                    }
                }
                thread::sleep(time::Duration::from_micros(10));
            }
        });
        tx_audio
    }
}
