use std::collections::HashMap;
use hlua::AnyLuaValue;
use nalgebra::{Point3, UnitQuaternion};

pub mod gameobject;
pub mod player;

pub enum GameCommand{
    SetGameObjectPosition(String, Point3<f32>),
    SetGameObjectRotation(String, UnitQuaternion<f32>),
    CallEvent(String, Vec<AnyLuaValue>),
    SpawnGameObject(gameobject::GameObject),
    GetGameObjectPosition(String),
    GetGameObjectRotation(String),
    SendLua(String, u32),
    GetObjects()
}

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

    pub fn update(&mut self){

    }

    pub fn fixed_update(&mut self){

    }
}
