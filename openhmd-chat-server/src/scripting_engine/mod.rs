use hlua;
use hlua::Lua;
use game::Game;
use std::{thread};
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};
use game::gameobject::{GameObjectBuilder, GameObject};

lazy_static! {
    pub static ref LUA_CHANNL_OUT: (Arc<Mutex<Sender<LuaEvent>>>, Arc<Mutex<Receiver<LuaEvent>>>) = {
        let channels = channel::<LuaEvent>();
        (Arc::new(Mutex::new(channels.0)), Arc::new(Mutex::new(channels.1)) )
    };
    pub static ref LUA_CHANNL_IN: (Arc<Mutex<Sender<LuaCommand>>>, Arc<Mutex<Receiver<LuaCommand>>>) = {
        let channels = channel::<LuaCommand>();
        (Arc::new(Mutex::new(channels.0)), Arc::new(Mutex::new(channels.1)) )
    };
}

pub enum LuaEvent{
    SetGameObjectPosition(String, (f32, f32, f32)),
    SpawnGameObject(GameObject),
    GetGameObjectPosition(String),
    GetObjects()
}

pub enum LuaCommand{
    GetGameObjectPosition(Vec<f32>),
    GetObjects(Vec<LuaEntity>),
}

pub struct LuaEntity{
    pub name: String
}

implement_lua_read!(LuaEntity);
implement_lua_push!(LuaEntity, |mut metatable| {
    let mut index = metatable.empty_array("__index");
    index.set("set_position", hlua::function4(|ent: &mut LuaEntity, x: f32, y: f32, z: f32|
        {
            let channels = LUA_CHANNL_OUT.0.lock().unwrap();
            let _ = channels.send(LuaEvent::SetGameObjectPosition(ent.name.clone(), (x, y, z)));
        }
    ));
    index.set("name", hlua::function1(|ent: &mut LuaEntity|
        ent.name.clone()
    ));
    index.set("get_position", hlua::function1(|ent: &mut LuaEntity|
        {
            let channels = LUA_CHANNL_OUT.0.lock().unwrap();
            let _ = channels.send(LuaEvent::GetGameObjectPosition(ent.name.clone()));
            let channels = LUA_CHANNL_IN.1.lock().unwrap();
            let data = channels.recv();
            if data.is_ok(){
                let data = data.unwrap();
                match data{
                    LuaCommand::GetGameObjectPosition(pos) => {
                        pos
                    }
                    _ => {
                        vec![0.0, 0.0, 0.0]
                    }
                }
            }
            else{
                vec![0.0, 0.0, 0.0]
            }
        }
    ));
});


pub struct ScriptingEngine{
    pub rx: Receiver<LuaEvent>
}

impl ScriptingEngine{
    pub fn new() -> ScriptingEngine{
        use nalgebra::geometry::{Point3};
        let (tx, rx) = channel::<LuaEvent>();
        thread::spawn(move || {
            let mut lua = Lua::new();
            //init
            lua.openlibs();
            {
                let mut world = lua.empty_array("World");
                world.set("create_game_object", hlua::function0(|| GameObjectBuilder::new() ));
                world.set("get_game_object", hlua::function1(|name: String| LuaEntity{name} ));
                world.set("get_all_objects", hlua::function0(|| {
                    let channels = LUA_CHANNL_OUT.0.lock().unwrap();
                    let _ = channels.send(LuaEvent::GetObjects());
                    let channels = LUA_CHANNL_IN.1.lock().unwrap();
                    let data = channels.recv();
                    if data.is_ok(){
                        let data = data.unwrap();
                        match data{
                            LuaCommand::GetObjects(objects) => objects,
                            _ => vec![]
                        }
                    }
                    else{
                        vec![]
                    }
                } ));
            }
            match lua.execute_from_reader::<(), _>(File::open(&Path::new("./assets/lua/test.lua")).unwrap()){
                Ok(x) => {},
                Err(err) => { println!("LUA ERROR: {}", err.description()); }
            };
        });
        ScriptingEngine{
            rx
        }
    }
    pub fn update(&mut self, game: &mut Game){
        use nalgebra::Point3;
        let channels = LUA_CHANNL_OUT.1.lock().unwrap();
        let data = channels.try_iter();
        for x in data{
            match x{
                LuaEvent::SpawnGameObject(game_object) => {
                    game.spawn_game_object(game_object);
                },
                LuaEvent::SetGameObjectPosition(name, pos) => {
                    if let Some(x) = game.gameobjects.get_mut(&name){
                        x.set_position(Point3::new(pos.0, pos.1, pos.2))
                    }
                },
                LuaEvent::GetGameObjectPosition(name) => {
                    if let Some(x) = game.gameobjects.get_mut(&name){
                        let channels = LUA_CHANNL_IN.0.lock().unwrap();
                        let _ = channels.send(LuaCommand::GetGameObjectPosition(vec![x.position[0], x.position[1], x.position[2]]));
                    }
                    else{
                        let channels = LUA_CHANNL_IN.0.lock().unwrap();
                        let _ = channels.send(LuaCommand::GetGameObjectPosition(vec![0.0, 0.0, 0.0]));
                    }
                },
                LuaEvent::GetObjects() => {
                    let mut objects = vec![];
                    for (name, x) in &game.gameobjects{
                        objects.push(LuaEntity{name: name.clone()});
                    }
                    let channels = LUA_CHANNL_IN.0.lock().unwrap();
                    let _ = channels.send(LuaCommand::GetObjects(objects));
                },
                _ => {}
            }
        }
    }
}
