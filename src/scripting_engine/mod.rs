use hlua;
use std::fs;
use hlua::{Lua, AnyLuaValue};
use game::Game;
use std::io::copy;
use std::{thread};
use std::fs::File;
use std::path::Path;
use render::Window;
use std::error::Error;
use std::sync::{Arc, Mutex};
use reqwest;
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
    CallEvent(String, Vec<AnyLuaValue>),
    SpawnGameObject(GameObject),
    GetGameObjectPosition(String),
    LoadModel(String, String),
    DownloadFile(String),
    RunLuaFile(String),
    RunLua(String),
    GetObjects()
}

pub enum LuaCommand{
    GetGameObjectPosition(Vec<f32>),
    GetObjects(Vec<LuaEntity>),
}

pub enum LuaLocalCommand{
    RunLua(String),
    RunLuaFile(String),
    CallEvent(String, Vec<AnyLuaValue>)
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
    tx: Sender<LuaLocalCommand>
}

impl ScriptingEngine{
    pub fn new() -> ScriptingEngine{
        use time::Duration;
        let (tx, rx) = channel::<LuaLocalCommand>();
        thread::spawn(move || {
            let mut lua = Lua::new();
            //init
            lua.openlibs();
            lua.set("run_lua_file", hlua::function1(|path: String| {
                let channels = LUA_CHANNL_OUT.0.lock().unwrap();
                let _ = channels.send(LuaEvent::RunLuaFile(path));
            } ));
            {
                let mut world = lua.empty_array("World");
                world.set("create_game_object", hlua::function0(|| GameObjectBuilder::new() ));
                world.set("load_model", hlua::function2(|path: String, name: String|{
                    let channels = LUA_CHANNL_OUT.0.lock().unwrap();
                    let _ = channels.send(LuaEvent::LoadModel(path, name));
                }));
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
            {
                let mut network = lua.empty_array("Network");
                network.set("download_file", hlua::function1(|url: String| {
                    let channels = LUA_CHANNL_OUT.0.lock().unwrap();
                    let _ = channels.send(LuaEvent::DownloadFile(url));
                } ));
            }
            let paths = fs::read_dir("./assets/lua/").unwrap();
            for path in paths {
                match lua.execute_from_reader::<(), _>(File::open(path.unwrap().path()).unwrap()){
                    Ok(_) => {},
                    Err(err) => { println!("LUA ERROR: {}", err.description()); }
                };
            }
            loop{
                let data = rx.try_iter();
                for x in data{
                    match x{
                        LuaLocalCommand::RunLua(script) => {
                            match lua.execute::<()>(&script){
                                Ok(_) => {},
                                Err(err) => { println!("LUA ERROR: {}", err.description()); }
                            };
                        }
                        LuaLocalCommand::RunLuaFile(path) => {
                            match lua.execute_from_reader::<(), _>(File::open(path).unwrap()){
                                Ok(_) => {},
                                Err(err) => { println!("LUA ERROR: {}", err.description()); }
                            };
                        }
                        LuaLocalCommand::CallEvent(name, args) => {
                            let mut call_event_fn: Option<hlua::LuaFunction<_>> = lua.get("CallEvent");
                            if let Some(mut call_event) = call_event_fn{
                                let result: Option<hlua::AnyLuaValue> = match call_event.call_with_args((name, args)) {
                                    Ok(res) => {Some(res)},
                                    Err(err) => {
                                        println!("LUA ERROR: {:?}", err);
                                        None
                                    },
                                };
                            }
                            else{
                                println!("Cannot call CallEvent function. Does events.lua properly loaded?");
                            }
                        }
                    }
                }
                thread::sleep(Duration::from_millis(1))
            }
        });
        ScriptingEngine{
            tx
        }
    }
    pub fn update(&mut self, game: &mut Game, window: &mut Window){
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
                    for (name, _) in &game.gameobjects{
                        objects.push(LuaEntity{name: name.clone()});
                    }
                    let channels = LUA_CHANNL_IN.0.lock().unwrap();
                    let _ = channels.send(LuaCommand::GetObjects(objects));
                },
                LuaEvent::RunLua(script) => {
                    let _ = self.tx.send(LuaLocalCommand::RunLua(script));
                }
                LuaEvent::RunLuaFile(path) => {
                    let _ = self.tx.send(LuaLocalCommand::RunLuaFile(path));
                }
                LuaEvent::DownloadFile(url) => {
                    let mut response = reqwest::get(&url).unwrap();
                    let mut dest = {
                        let fname = Path::new(response
                            .url()
                            .path_segments()
                            .and_then(|segments| segments.last())
                            .and_then(|name| if name.is_empty() { None } else { Some(name) })
                            .unwrap_or("tmp.bin"));
                        let extension = fname.extension().unwrap().to_str().unwrap();
                        let dir = match extension{
                            "obj" => {
                                Some(Path::new("models/"))
                            },
                            "mtl" => {
                                Some(Path::new("models/"))
                            },
                            "png" => {
                                Some(Path::new("textures/"))
                            },
                            "jpg" => {
                                Some(Path::new("textures/"))
                            },
                            "lua" => {
                                println!("test");
                                Some(Path::new("temp/"))
                            },
                            _ => {
                                None
                            }
                        };
                        match dir{
                            Some(dir) => {
                                let mut path = Path::new("./assets/").join(dir);
                                Some(File::create(path.join(fname)).unwrap())
                            }
                            None => {
                                None
                            }
                        }
                    };
                    if let Some(mut dest) = dest{
                        copy(&mut response, &mut dest).unwrap();
                    }
                }
                LuaEvent::CallEvent(name, args) => {
                    let _ = self.tx.send(LuaLocalCommand::CallEvent(name, args));
                },
                LuaEvent::LoadModel(path, name) => {
                    let _ = window.load_model_and_push(path, name, (0.1, 0.1, 0.1));
                },
                _ => {}
            }
        }
    }
}
