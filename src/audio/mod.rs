use alto;
use opus;
use alto::*;
use alto::efx::{EaxReverbEffect, REVERB_PRESET_GENERIC};
use player;

use std::sync::Arc;
use std::option::Option;
use std::sync::mpsc::channel;
use std::sync::mpsc;
use std::collections::HashMap;

pub struct Audio_Wrapper{
    pub alto: Alto,
    pub dev: alto::OutputDevice,
    pub dev_cap: alto::Capture<Mono<i16>>,
    pub context: alto::Context,
    pub player_position: (f32,f32,f32),
    pub player_rotation: (f32,f32,f32)
}

pub struct Audio_Static_Source{
    pub source: StreamingSource
}

pub struct AudioMsg {
    pub data: Vec<u8>,
    pub player_position: (f32,f32,f32),
    pub player_rotation: (f32,f32,f32),
    pub source_id: u32
}

impl Audio_Wrapper{
    pub fn new() -> Result<Audio_Wrapper, &'static str>{
        let mut alto = match Alto::load_default(){
            Ok(x) => x,
            _ => {
                return Err("Failed to load OpenAL");
            }
        };
        let mut dev = match alto.open(None){
            Ok(x) => x,
            _ => {
                return Err("Failed to open OpenAL default device");
            }
        };
        let mut ctx = match dev.new_context(None){
            Ok(x) => x,
            _ => {
                return Err("Failed to open context");
            }
        };
        let mut dev_cap : alto::Capture<Mono<i16>> = match alto.open_capture(None, 16000, 2048){
            Ok(x) => x,
            _ => {
                return Err("Failed to open OpenAL default capture device");
            }
        };

        Ok(Audio_Wrapper{
            alto: alto,
            dev: dev,
            dev_cap: dev_cap,
            context: ctx,
            player_position: (0.0, 0.0, 0.0),
            player_rotation: (0.0, 0.0, 0.0),
        })
    }
    pub fn create_static_source(&self) -> Result<Audio_Static_Source, &'static str>{
        let mut slot = match self.dev.is_extension_present(alto::ext::Alc::Efx) {
            true => {
                let mut slot = self.context.new_aux_effect_slot().unwrap();
                let mut reverb: EaxReverbEffect = self.context.new_effect().unwrap();
                reverb.set_preset(&REVERB_PRESET_GENERIC).unwrap();
                slot.set_effect(&reverb).unwrap();
                Some(slot)
            }
            false => {
                None
            }
        };
        let mut src = match self.context.new_streaming_source(){
            Ok(x) => x,
            _ => {
                return Err("Failed to create static audio source");
            }
        };
        if slot.is_some(){
            //src.set_aux_send(0, &mut slot.unwrap()).unwrap();
        }
        Ok(Audio_Static_Source{
            source: src
        })
    }
    pub fn start_capture(&mut self){
        self.dev_cap.start();
    }
    pub fn samples_len(&self) -> i32{
        self.dev_cap.samples_len()
    }
}

impl Audio_Static_Source{

}

pub fn start_audio(tx: &mpsc::Sender<AudioMsg>, rx: &mpsc::Receiver<AudioMsg>, rx_players: &mpsc::Receiver<HashMap<u32, player::Player>>){
    //WARNING: Very poor code
    use std::collections::{VecDeque, HashMap};
    use std::thread;
    println!("Init audio_wrapper");
    let audio_wrapper = Audio_Wrapper::new();
    if audio_wrapper.is_ok(){
        let mut opus_encoder = opus::Encoder::new(16000, opus::Channels::Mono, opus::Application::Voip).unwrap();
        let mut opus_decoder = opus::Decoder::new(16000, opus::Channels::Mono).unwrap();

        let mut audio_wrapper = audio_wrapper.unwrap();

        let mut sources = HashMap::new();
        let mut src = audio_wrapper.create_static_source().unwrap().source;
        sources.insert(0, src);

        let mut buffer_queue : VecDeque<alto::Buffer> = VecDeque::<alto::Buffer>::new();
        let mut buffer: Vec<alto::Mono<i16>> = vec![alto::Mono::<i16> { center : 0 }; 320 as usize];

        audio_wrapper.start_capture();

        loop{
            //Creating new buffer for holding mic output
            let mut buffer: Vec<alto::Mono<i16>> = vec![alto::Mono::<i16> { center : 0 }; 320 as usize];
            //Check if we ready for collect
            let mut samples_len = audio_wrapper.samples_len();
            while samples_len < buffer.len() as i32{
                samples_len = audio_wrapper.samples_len();
            }
            //Collecting samples
            audio_wrapper.dev_cap.capture_samples(&mut buffer).unwrap();

            //Opus encoding
            let encode_buf = buffer.iter().map(|&x| x.center).collect::<Vec<_>>();
            let encoded = opus_encoder.encode_vec(&encode_buf, 4000).unwrap();
            tx.send(AudioMsg{
                data: encoded,
                player_position: audio_wrapper.player_position,
                player_rotation: audio_wrapper.player_rotation,
                source_id: 0,
            });
            let playerslist = rx_players.try_iter();
            for x in playerslist{
                for (id, player) in x{
                    let mut src = audio_wrapper.create_static_source().unwrap().source;
                    sources.entry(id).or_insert(src);
                    let src = sources.get_mut(&id).unwrap();
                    let (posx, posy, posz) = player.position;
                    src.set_position([posx, posy, posz]);
                }
            }
            let data = rx.try_iter();
            for data in data{
                let src = sources.get_mut(&data.source_id);
                if src.is_some(){
                    let src = src.unwrap();
                    //unqueue buffers
                    let mut buffers_avail = src.buffers_processed();
                    while buffers_avail > 0 {
                        let buf = src.unqueue_buffer().unwrap();
                        buffer_queue.push_back( buf );
                        buffers_avail = buffers_avail - 1;
                    }

                    audio_wrapper.player_position = data.player_position;
                    let (posx, posy, posz) = audio_wrapper.player_position;
                    let (rotx, roty, rotz) = audio_wrapper.player_rotation;
                    let data = data.data;
                    audio_wrapper.context.set_position([-posx, -posy, -posz]);
                    audio_wrapper.context.set_orientation(([1.0,1.0,1.0], [rotx, roty, rotz])).unwrap();

                    let mut decode = vec![0i16; 320];
                    opus_decoder.decode(&data, &mut decode, false).unwrap();
                    let mut empty_buffer: Vec<alto::Mono<i16>> = vec![alto::Mono::<i16> { center : 0 }; 320 as usize];
                    for (num, x) in decode.iter().enumerate(){
                        empty_buffer[num].center = *x;
                    }
                    //Creating AL buffer
                    let buf = audio_wrapper.context.new_buffer::<alto::Mono<i16>,_>(&empty_buffer, 16000).unwrap();
                    buffer_queue.push_back(buf);
                    //Check if buffer not empty
                    if buffer_queue.len() > 0 {
                        //Pushing buffer in src
                        let mut buf = buffer_queue.pop_front().unwrap();
                        buf.set_data(&empty_buffer, 16000).unwrap();
                        src.queue_buffer(buf);
                        //Playing sound from buffer
                        if src.state() != alto::SourceState::Playing {
                            src.play()
                        }
                    }
                }
                else{
                    println!("Looks like a bug");
                }
            }
        }
    }
}
