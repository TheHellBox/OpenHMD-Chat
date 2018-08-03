use std::collections::HashMap;

pub mod gameobject;


pub struct Game{
    pub gameobjects: HashMap<String, gameobject::GameObject>
}
impl Game{
    pub fn new() -> Game{
        Game{
            gameobjects: HashMap::new()
        }
    }
    pub fn add_game_object(&mut self, go: gameobject::GameObject){
        self.gameobjects.insert(go.name.clone(), go);
    }

    pub fn update(&mut self){

    }

    pub fn fixed_update(&mut self){

    }
}
