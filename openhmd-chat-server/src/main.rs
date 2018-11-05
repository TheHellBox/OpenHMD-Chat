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
use network::{MsgDst, MessageType};
use cobalt::MessageKind;

fn main() {
    let mut game = game::Game::new();
    let mut scripting_engine = scripting_engine::ScriptingEngine::new();
    let (mut network, mut net_rx) = network::Network::new();
    let mut net_tx = network.tx_in.clone();
    thread::spawn(move || {
        network.listen("0.0.0.0:4460");
        network.init();
    });
    loop{
        for x in net_rx.try_iter(){
            match x{
                network::NetworkCommand::SendGameObjects(id) => {
                    for (name, game_object) in &game.gameobjects{
                        net_tx.send((MessageKind::Reliable, MessageType::CreateGameObject(game_object.name.clone()), MsgDst::Id(id)));
                        net_tx.send((MessageKind::Reliable, MessageType::GameObjectChangedPosition(game_object.name.clone(), game_object.position), MsgDst::Id(id)));
                        net_tx.send((MessageKind::Reliable, MessageType::GameObjectChangedRotation(game_object.name.clone(), game_object.rotation), MsgDst::Id(id)));
                        net_tx.send((MessageKind::Reliable, MessageType::GameObjectChangedModel(game_object.name.clone(), game_object.render_object.clone()), MsgDst::Id(id)));
                    }
                },
                _ => {}
            }
        }
        scripting_engine.update(&mut game, &mut net_tx);
        {
            let channels = scripting_engine::LUA_CHANNL_OUT.0.lock().unwrap();
            let _ = channels.send(game::GameCommand::CallEvent("Update".to_string(), vec![]));
        }
        thread::sleep(time::Duration::from_millis(1));
    }
}
