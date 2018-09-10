use render::Window;
use std::collections::HashMap;
use network::{NetworkCommand, NetworkEvent};
use nalgebra::{Point3, UnitQuaternion, Vector3};
use std::sync::mpsc::{Sender, Receiver};
use bincode::{deserialize, serialize};
use glium::glutin::{Event, WindowEvent, VirtualKeyCode, ElementState};

pub mod gameobject;

#[derive(Clone)]
pub struct Game{
    pub gameobjects: HashMap<String, gameobject::GameObject>,
    player_rotation: f32
}

impl Game{
    pub fn new() -> Game{
        Game{
            gameobjects: HashMap::new(),
            player_rotation: 0.0
        }
    }
    pub fn spawn_game_object(&mut self, go: gameobject::GameObject){
        self.gameobjects.insert(go.name.clone(), go);
    }

    pub fn update(&mut self, net_rx: &mut Receiver<NetworkCommand>, net_tx: &mut Sender<NetworkEvent>, window: &mut Window){
        use support::direction;
        for event in window.events.clone(){
            match event{
                Event::WindowEvent{window_id: _, event: window_event} => {
                    match window_event{
                        WindowEvent::KeyboardInput{device_id: _, input} => {
                            let key = input.virtual_keycode;
                            let state = match input.state{
                                ElementState::Pressed => true,
                                _ => false
                            };
                            if let Some(key) = key{
                                match key{
                                    VirtualKeyCode::Q => {
                                        if !state {
                                            let mut rotation = UnitQuaternion::from_euler_angles(0.0, -0.7853982, 0.0);
                                            window.character_view.rotation *= rotation;
                                        }
                                    },
                                    VirtualKeyCode::E => {
                                        if !state {
                                            let mut rotation = UnitQuaternion::from_euler_angles(0.0, 0.7853982, 0.0);
                                            window.character_view.rotation *= rotation;
                                        }
                                    },
                                    VirtualKeyCode::W => {
                                        if !state {
                                            window.character_view.position.vector -= direction(window.head_dir, Vector3::new(0.0, 0.0, 1.0)) / 3.0;
                                            let position = window.character_view.position.vector;
                                            let position = Point3::new(position[0], position[1], position[2]);
                                            let _ = net_tx.send(NetworkEvent::SendPosition(position));
                                        }
                                    },
                                    VirtualKeyCode::S => {
                                        if !state {
                                            window.character_view.position.vector += direction(window.head_dir, Vector3::new(0.0, 0.0, 1.0)) / 3.0;
                                            let position = window.character_view.position.vector;
                                            let position = Point3::new(position[0], position[1], position[2]);
                                            let _ = net_tx.send(NetworkEvent::SendPosition(position));
                                        }
                                    },
                                    VirtualKeyCode::A => {
                                        if !state {
                                            window.character_view.position.vector -= direction(window.head_dir, Vector3::new(1.0, 0.0, 0.0)) / 3.0;
                                            let position = window.character_view.position.vector;
                                            let position = Point3::new(position[0], position[1], position[2]);
                                            let _ = net_tx.send(NetworkEvent::SendPosition(position));
                                        }
                                    },
                                    VirtualKeyCode::D => {
                                        if !state {
                                            window.character_view.position.vector += direction(window.head_dir, Vector3::new(1.0, 0.0, 0.0)) / 3.0;
                                            let position = window.character_view.position.vector;
                                            let position = Point3::new(position[0], position[1], position[2]);
                                            let _ = net_tx.send(NetworkEvent::SendPosition(position));
                                        }
                                    },
                                    _ => {}
                                }
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
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
