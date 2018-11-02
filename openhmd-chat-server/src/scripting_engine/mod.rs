pub mod std_lib;

use hlua;
use std::fs;
use game::Game;
use std::{thread};
use std::fs::File;
use std::path::Path;
use std::error::Error;
use cobalt::MessageKind;
use std::sync::{Arc, Mutex};
use hlua::{Lua, AnyLuaValue};
use network::{MessageType, MsgDst};
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
    SendLua(String, u32),
    GetObjects()
}

pub enum LuaCommand{
    GetGameObjectPosition(Vec<f32>),
    GetObjects(Vec<LuaEntity>),
}

pub enum LuaLocalCommand{
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

            // Open std libs

            lua.execute::<()>(std_lib::LUA_STD_LIB_EVENTS).unwrap();

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
            {
                let mut network = lua.empty_array("Network");
                network.set("SendLua", hlua::function2(|script: String, id: u32|{
                    let channels = LUA_CHANNL_OUT.0.lock().unwrap();
                    let _ = channels.send(LuaEvent::SendLua(script, id));
                }));
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
    pub fn update(&mut self, game: &mut Game, net_tx: &mut Sender<(MessageKind, MessageType, MsgDst)>){
        use nalgebra::Point3;
        let channels = LUA_CHANNL_OUT.1.lock().unwrap();
        let data = channels.try_iter();
        for x in data{
            match x{
                LuaEvent::SpawnGameObject(game_object) => {
                    net_tx.send((MessageKind::Reliable, MessageType::CreateGameObject(game_object.name.clone()), MsgDst::Boardcast()));
                    net_tx.send((MessageKind::Reliable, MessageType::GameObjectChangedPosition(game_object.name.clone(), game_object.position), MsgDst::Boardcast()));
                    net_tx.send((MessageKind::Reliable, MessageType::GameObjectChangedRotation(game_object.name.clone(), game_object.rotation), MsgDst::Boardcast()));
                    net_tx.send((MessageKind::Reliable, MessageType::GameObjectChangedModel(game_object.name.clone(), game_object.render_object.clone()), MsgDst::Boardcast()));
                    game.spawn_game_object(game_object);
                },
                LuaEvent::SetGameObjectPosition(name, pos) => {
                    if let Some(x) = game.gameobjects.get_mut(&name){
                        x.set_position(Point3::new(pos.0, pos.1, pos.2))
                    }
                    net_tx.send((MessageKind::Instant, MessageType::GameObjectChangedPosition(name, Point3::new(pos.0, pos.1, pos.2)), MsgDst::Boardcast()));
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
                LuaEvent::SendLua(script, id) => {
                    net_tx.send((MessageKind::Reliable, MessageType::LuaScript(script), MsgDst::Id(id)));
                },
                LuaEvent::CallEvent(name, args) => {
                    let _ = self.tx.send(LuaLocalCommand::CallEvent(name, args));
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
