use alto;
use opus;
use alto::*;
use alto::efx::{EaxReverbEffect, REVERB_PRESET_GENERIC};

use std::sync::Arc;
use std::option::Option;
use std::sync::mpsc::channel;
use std::sync::mpsc;

pub struct Audio_Wrapper{
    pub alto: Alto,
    pub dev: alto::OutputDevice,
    pub dev_cap: alto::Capture<Mono<i16>>,
    pub context: alto::Context
}

pub struct Audio_Static_Source{
    pub source: StreamingSource
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
        let mut dev_cap : alto::Capture<Mono<i16>> = match alto.open_capture(None, 16000, 1024){
            Ok(x) => x,
            _ => {
                return Err("Failed to open OpenAL default capture device");
            }
        };

        Ok(Audio_Wrapper{
            alto: alto,
            dev: dev,
            dev_cap: dev_cap,
            context: ctx
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

pub fn start_audio(tx: &mpsc::Sender<(Vec<u8>)>, rx: &mpsc::Receiver<(Vec<u8>)>){
    use std::collections::VecDeque;
    use std::thread;

    let audio_wrapper = Audio_Wrapper::new();
    if audio_wrapper.is_ok(){
        let mut opus_encoder = opus::Encoder::new(16000, opus::Channels::Mono, opus::Application::Voip).unwrap();
        let mut opus_decoder = opus::Decoder::new(16000, opus::Channels::Mono).unwrap();
        let mut audio_wrapper = audio_wrapper.unwrap();
        let mut src = audio_wrapper.create_static_source().unwrap().source;
        let mut buffer_queue : VecDeque<alto::Buffer> = VecDeque::<alto::Buffer>::new();
        let mut buffer: Vec<alto::Mono<i16>> = vec![alto::Mono::<i16> { center : 0 }; 320 as usize];

        audio_wrapper.start_capture();

        loop{
            //unqueue buffers
            let mut buffers_avail = src.buffers_processed();
            while buffers_avail > 0 {
                let buf = src.unqueue_buffer().unwrap();
                buffer_queue.push_back( buf );
                buffers_avail = buffers_avail - 1;
            }
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
            tx.send(encoded);
            let data = rx.try_iter();
            for data in data{
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
        }
    }
}
