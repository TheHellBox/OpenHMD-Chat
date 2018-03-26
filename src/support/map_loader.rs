use json;
use rand;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Index;
use rand::Rng;
use bytevec::{ByteEncodable, ByteDecodable};

#[derive(PartialEq, Debug, Default, Clone)]
pub struct MapObject{
    pub position: (f32,f32,f32),
    pub rotation: (f32,f32,f32,f32),
    pub model: String,
    pub texture: String
}
bytevec_impls! {
    impl MapObject {
        position: (f32,f32,f32),
        rotation: (f32,f32,f32,f32),
        model: String,
        texture: String
    }
}
#[derive(PartialEq, Debug, Default, Clone)]
pub struct Collider{
    pub position: (f32,f32,f32),
    pub scale: (f32,f32,f32),
}
bytevec_impls! {
    impl Collider {
        position: (f32,f32,f32),
        scale: (f32,f32,f32)
    }
}

pub struct Map{
    pub objects: HashMap<u32, MapObject>,
    pub colliders: HashMap<u32, Collider>,
}
impl Map{
    pub fn new() -> Map{
        Map{
            objects: HashMap::new(),
            colliders: HashMap::new()
        }
    }
    pub fn load(&mut self, content: &String){
        let parsed = json::parse(&content);
        if parsed.is_ok(){
            let parsed = parsed.unwrap();
            for (text, x) in parsed["objects"].entries(){
                //FIXME: A lot of unwraps, can cause crash.
                let pos = (x["position"][0].as_f32().unwrap(), x["position"][1].as_f32().unwrap(), x["position"][2].as_f32().unwrap());
                let rot = (x["rotation"][0].as_f32().unwrap(), x["rotation"][1].as_f32().unwrap(), x["rotation"][2].as_f32().unwrap(), x["rotation"][3].as_f32().unwrap());
                let model = x["model"].as_str().unwrap().to_string();
                let tex = x["texture"].as_str().unwrap().to_string();
                let object = MapObject::new(pos, rot, model, tex);
                self.objects.insert(rand::thread_rng().gen_range(10000, 900000), object);
            }
            for (text, x) in parsed["colliders"].entries(){
                let pos = (x["position"][0].as_f32().unwrap(), x["position"][1].as_f32().unwrap(), x["position"][2].as_f32().unwrap());
                let scale = (x["scale"][0].as_f32().unwrap(), x["scale"][1].as_f32().unwrap(), x["scale"][2].as_f32().unwrap());
                let collider = Collider{
                    position: pos,
                    scale: scale
                };
                self.colliders.insert(rand::thread_rng().gen_range(0, 900000), collider);
            }
        }
        else{
            println!("Error while loading map");
        }
    }
    pub fn objects(&self) -> &HashMap<u32, MapObject>{
        &self.objects
    }
    pub fn colliders(&self) -> &HashMap<u32, Collider>{
        &self.colliders
    }
}
impl MapObject{
    pub fn new(pos: (f32,f32,f32), rot: (f32,f32,f32,f32), model: String, texture: String) -> MapObject{
        MapObject{
            position: pos,
            rotation: rot,
            model: model,
            texture: texture,
        }
    }
    pub fn to_network(&self) -> Vec<u8>{
        self.encode::<u8>().unwrap()
    }
    pub fn from_network(message: Vec<u8>) -> MapObject{
        MapObject::decode::<u8>(&message).unwrap()
    }
}

impl Collider{
    pub fn to_network(&self) -> Vec<u8>{
        self.encode::<u8>().unwrap()
    }
    pub fn from_network(message: Vec<u8>) -> Collider{
        Collider::decode::<u8>(&message).unwrap()
    }
}
