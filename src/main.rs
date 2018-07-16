
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate glium;

extern crate tobj;
extern crate clap;
extern crate opus;
extern crate alto;
extern crate serde;
extern crate image;
extern crate cobalt;
extern crate bincode;
extern crate nalgebra;
extern crate openhmd_rs;

mod audio;
mod render;
mod network;
mod support;

use nalgebra::geometry::{Point3, UnitQuaternion, Quaternion, Translation3};
use std::{thread, time};
use clap::{Arg, App};

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
        .arg(Arg::with_name("vr")
            .short("v")
            .long("vr")
            .help("VR mode"))
        .get_matches();

    let mut vr_mode = false;

    let addr = matches.value_of("addr").unwrap_or("127.0.0.1:4460").to_string();
    let vr = matches.values_of_lossy("vr");
    if vr.is_some(){
        vr_mode = true;
    }

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

    let mut window = render::Window::new(1024, 768, "Test", vr_mode);
    window.init();

    let test_model = window.load_model("./assets/models/scene.obj".to_string());
    window.add_draw_object(test_model, Point3::new(0.0, 5.0, 0.0), UnitQuaternion::from_quaternion(Quaternion::new(0.0, 0.707, 0.0, 0.707)), (1.0, 1.0, 1.0));
    loop{
        window.draw();
        window.update_vr();
        thread::sleep(time::Duration::from_millis(1));
    }
}
