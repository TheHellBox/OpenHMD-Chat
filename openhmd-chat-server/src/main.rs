#[macro_use]
extern crate hlua;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

extern crate rand;
extern crate serde;
extern crate cobalt;
extern crate bincode;
extern crate nalgebra;

mod game;
mod audio;
mod network;
mod support;
mod scripting_engine;

use std::{thread, time};

fn main() {
    let mut game = game::Game::new();
    let mut scripting_engine = scripting_engine::ScriptingEngine::new();
    thread::spawn(move || {
        let mut network = network::Network::new();
        network.listen("0.0.0.0:4460");
        network.init();
    });
    loop{
        thread::sleep(time::Duration::from_millis(1));
        scripting_engine.update(&mut game);
    }
}
