#[macro_use]
extern crate serde_derive;

extern crate clap;
extern crate opus;
extern crate alto;
extern crate serde;
extern crate cobalt;
extern crate bincode;

mod audio;
mod network;

use std::{thread, time};
use clap::{Arg, App, SubCommand};

fn main() {
    println!("Hello, world!");

    let matches = App::new("OpenHMD-Chat")
        .version("0.1")
        .author("The HellBox <thehellbox11@gmail.com>")
        .about("Online chat for VR")
        .arg(Arg::with_name("addr")
            .short("a")
            .long("addr")
            .help("Sets addr to connect to")
            .takes_value(true))
        .get_matches();

    let addr = matches.value_of("addr").unwrap_or("127.0.0.1:4460").to_string();

    let frames = 1280;
    let sample_rate = 16000;

    let mut network = network::Network::new();
    let net_tx = network.tx.clone();

    let audio = audio::AudioWrapper::new().unwrap();
    let tx_audio = audio.init(sample_rate, frames, net_tx);

    thread::spawn(move || {
        network.connect(addr);
        network.init(tx_audio.clone());
    });

    loop{
        thread::sleep(time::Duration::from_millis(1));
    }
}
