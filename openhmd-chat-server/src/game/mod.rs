use std::collections::HashMap;
use hlua::AnyLuaValue;
use nalgebra::{Point3, Translation3, UnitQuaternion, Vector3};

use ncollide3d::shape::{ShapeHandle};
use std::sync::mpsc::{Sender};
use network::{MessageType, MsgDst};
use nphysics3d::world::World;
use cobalt::MessageKind;

pub mod collider_builder;
pub mod raycasting;
pub mod gameobject;
pub mod player;

pub enum GameCommand{
    SetGameObjectPosition(String, Point3<f32>),
    SetGameObjectRotation(String, UnitQuaternion<f32>),
    CreateRigidBody(ShapeHandle<f32>, bool),
    CallEvent(String, Vec<AnyLuaValue>),
    SpawnGameObject(gameobject::GameObject),
    AttachCollider(String, collider_builder::LuaCollider),
    GetGameObjectPosition(String),
    MakeRaycast(Point3<f32>, Vector3<f32>),
    GetGameObjectRotation(String),
    ChangePlayersCameraPosition(u32, Point3<f32>),
    ChangePlayersCameraRotation(u32, UnitQuaternion<f32>),
    SendLua(String, u32),
    RemoveGameObject(String),
    GetObjects()
}

pub struct Game{
    pub gameobjects: HashMap<String, gameobject::GameObject>,
    pub physic_world: World<f32>,
}

impl Game{
    pub fn new() -> Game{
        let mut physic_world = World::new();
        physic_world.set_gravity(Vector3::new(0.0, -9.81, 0.0));
        physic_world.set_timestep(0.004);
        Game{
            gameobjects: HashMap::new(),
            physic_world
        }
    }
    pub fn spawn_game_object(&mut self, go: gameobject::GameObject){
        self.gameobjects.insert(go.name.clone(), go);
    }
    pub fn remove_game_object(&mut self, gameobject_name: String){
        self.gameobjects.remove(&gameobject_name);
    }
    pub fn update(&mut self, net_tx: &mut Sender<(MessageKind, MessageType, MsgDst)>){
        self.physic_world.step();
        for (name, game_object) in &mut self.gameobjects {
            if let Some(physic_body) = game_object.physic_body{
                let body = self.physic_world.body_part(physic_body);
                if body.is_active() == true {
                    let position = body.position();
                    let rotation = position.rotation.euler_angles();
                    let rotation = UnitQuaternion::from_euler_angles(rotation.0, rotation.1, rotation.2);

                    let position = position.translation.vector;
                    let position = Point3::new(position[0], position[1], position[2]);
                    game_object.set_position(position);
                    game_object.set_rotation_unit(rotation);
                    let _ = net_tx.send((MessageKind::Instant, MessageType::GameObjectChangedPosition(name.clone(), position), MsgDst::Boardcast()));
                    let _ = net_tx.send((MessageKind::Instant, MessageType::GameObjectChangedRotation(name.clone(), rotation), MsgDst::Boardcast()));
                }
            }
        }
    }
}
