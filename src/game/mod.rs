use render::Window;
use std::collections::HashMap;
use network::{NetworkCommand, NetworkEvent};
use nalgebra::{Point3, UnitQuaternion};
use std::sync::mpsc::{Sender, Receiver};
use bincode::{deserialize, serialize};

pub mod gameobject;

#[derive(Clone)]
pub struct Game{
    pub gameobjects: HashMap<String, gameobject::GameObject>
}

impl Game{
    pub fn new() -> Game{
        Game{
            gameobjects: HashMap::new()
        }
    }
    pub fn spawn_game_object(&mut self, go: gameobject::GameObject){
        self.gameobjects.insert(go.name.clone(), go);
    }

    pub fn update(&mut self, net_rx: &mut Receiver<NetworkCommand>, net_tx: &mut Sender<NetworkEvent>, window: &mut Window){
        for x in net_rx.try_iter(){
            match x{
                NetworkCommand::CreatePlayerGameobject(id) => {
                    println!("Creating player{}", id);
                    let player = gameobject::GameObjectBuilder::new()
                        .with_name(format!("player{}", id).to_string())
                        .with_rotation_unit(UnitQuaternion::from_euler_angles(0.0, -90.0, 0.0))
                        .with_render_object("cube".to_string())
                        .build();
                    self.spawn_game_object(player);
                },
                NetworkCommand::ChangeGameObjectPosition(name, position) => {
                    match self.gameobjects.get_mut(&name){
                        Some(x) => {
                            x.position = position;
                            println!("{}", position);
                        }
                        None => {
                            println!("Cannot find gameobject with name {}", name);
                        }
                    }
                },
                NetworkCommand::ChangeGameObjectRotation(name, rotation) => {
                    match self.gameobjects.get_mut(&name){
                        Some(x) => {
                            x.rotation = rotation;
                            println!("{}", rotation);
                        }
                        None => {
                            println!("Cannot find gameobject with name {}", name);
                        }
                    }
                },
                NetworkCommand::SendPlayerInfo() => {
                    println!("Sending client info...");
                    // Get players position from character view and transform it from Translation3 to Point3
                    let position = window.character_view.position.vector;
                    let position = Point3::new(position[0], position[1], position[2]);
                    // Get players rotation
                    let rotation = window.character_view.rotation;
                    // Send Position
                    let _ = net_tx.send(NetworkEvent::SendPosition(position));
                    let _ = net_tx.send(NetworkEvent::SendRotation(rotation));
                }
                _ => {}
            }
        }
    }

    pub fn fixed_update(&mut self){

    }
}
