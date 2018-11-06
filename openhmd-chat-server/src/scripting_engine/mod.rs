pub mod std_lib;

use hlua;
use std::fs;
use game::Game;
use std::{thread};
use std::fs::File;
use std::path::Path;
use nalgebra::{Point3, UnitQuaternion, Vector3};
use game::{player::LuaPlayer, GameCommand};
use std::error::Error;
use cobalt::MessageKind;
use std::sync::{Arc, Mutex};
use hlua::{Lua, AnyLuaValue};
use network::{MessageType, MsgDst};
use std::sync::mpsc::{channel, Sender, Receiver};
use game::gameobject::{GameObjectBuilder, GameObject};

lazy_static! {
    pub static ref LUA_CHANNL_OUT: (Arc<Mutex<Sender<GameCommand>>>, Arc<Mutex<Receiver<GameCommand>>>) = {
        let channels = channel::<GameCommand>();
        (Arc::new(Mutex::new(channels.0)), Arc::new(Mutex::new(channels.1)) )
    };
    pub static ref LUA_CHANNL_IN: (Arc<Mutex<Sender<LuaCommand>>>, Arc<Mutex<Receiver<LuaCommand>>>) = {
        let channels = channel::<LuaCommand>();
        (Arc::new(Mutex::new(channels.0)), Arc::new(Mutex::new(channels.1)) )
    };
}

pub enum LuaCommand{
    ReturnVec(Vec<f32>),
    GetObjects(Vec<LuaEntity>),
}

pub enum LuaLocalCommand{
    CallEvent(String, Vec<AnyLuaValue>)
}

pub struct LuaEntity{
    pub name: String
}

pub fn get_game_value(game_cmd: GameCommand) -> Vec<f32>{
    {
        let channels = LUA_CHANNL_OUT.0.lock().unwrap();
        let _ = channels.send(game_cmd);
    }
    let channels = LUA_CHANNL_IN.1.lock().unwrap();
    let data = channels.recv();
    if data.is_ok(){
        let data = data.unwrap();
        match data{
            LuaCommand::ReturnVec(pos) => {
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

implement_lua_read!(LuaEntity);
implement_lua_push!(LuaEntity, |mut metatable| {
    let mut index = metatable.empty_array("__index");
    index.set("SetPosition", hlua::function4(|ent: &mut LuaEntity, x: f32, y: f32, z: f32|
        {
            let channels = LUA_CHANNL_OUT.0.lock().unwrap();
            let _ = channels.send(GameCommand::SetGameObjectPosition(ent.name.clone(), Point3::new(x, y, z)));
        }
    ));
    index.set("SetRotation", hlua::function4(|ent: &mut LuaEntity, x: f32, y: f32, z: f32|
        {
            let channels = LUA_CHANNL_OUT.0.lock().unwrap();
            let _ = channels.send(GameCommand::SetGameObjectRotation(ent.name.clone(), UnitQuaternion::from_euler_angles(x, y, z)));
        }
    ));
    index.set("LookAt", hlua::function4(|ent: &mut LuaEntity, x: f32, y: f32, z: f32|
        {
            let channels = LUA_CHANNL_OUT.0.lock().unwrap();
            let _ = channels.send(GameCommand::SetGameObjectRotation(ent.name.clone(), UnitQuaternion::look_at_rh(&Vector3::new(x, y, z), &Vector3::y())));
        }
    ));
    index.set("Direction", hlua::function4(|ent: &mut LuaEntity, x: f32, y: f32, z: f32|
        {
            use support::direction;
            let rot = get_game_value(GameCommand::GetGameObjectRotation(ent.name.clone()));
            let rot = UnitQuaternion::from_euler_angles(rot[0], rot[1], rot[2]);
            let dir = direction(rot, Vector3::new(x, y, z));
            vec![dir[0], dir[1], dir[2]]
        }
    ));
    index.set("Name", hlua::function1(|ent: &mut LuaEntity|
        ent.name.clone()
    ));
    index.set("GetPosition", hlua::function1(|ent: &mut LuaEntity|
        {
            get_game_value(GameCommand::GetGameObjectPosition(ent.name.clone()))
        }
    ));
    index.set("GetRotation", hlua::function1(|ent: &mut LuaEntity|
        {
            get_game_value(GameCommand::GetGameObjectRotation(ent.name.clone()))
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
                world.set("CreateGameObject", hlua::function0(|| GameObjectBuilder::new() ));
                world.set("GetGameObject", hlua::function1(|name: String| LuaEntity{name} ));
                world.set("GetAllObjects", hlua::function0(|| {
                    let channels = LUA_CHANNL_OUT.0.lock().unwrap();
                    let _ = channels.send(GameCommand::GetObjects());
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
                    let _ = channels.send(GameCommand::SendLua(script, id));
                }));
            }
            {
                let mut player = lua.empty_array("Player");
                player.set("GetByID", hlua::function1(|id: u32|{
                    LuaPlayer{id}
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
        let channels = LUA_CHANNL_OUT.1.lock().unwrap();
        let data = channels.try_iter();
        for x in data{
            match x{
                GameCommand::SpawnGameObject(game_object) => {
                    net_tx.send((MessageKind::Ordered, MessageType::CreateGameObject(game_object.name.clone()), MsgDst::Boardcast()));
                    net_tx.send((MessageKind::Ordered, MessageType::GameObjectChangedPosition(game_object.name.clone(), game_object.position), MsgDst::Boardcast()));
                    net_tx.send((MessageKind::Ordered, MessageType::GameObjectChangedRotation(game_object.name.clone(), game_object.rotation), MsgDst::Boardcast()));
                    net_tx.send((MessageKind::Ordered, MessageType::GameObjectChangedModel(game_object.name.clone(), game_object.render_object.clone()), MsgDst::Boardcast()));
                    game.spawn_game_object(game_object);
                },
                GameCommand::SetGameObjectPosition(name, pos) => {
                    if let Some(x) = game.gameobjects.get_mut(&name){
                        x.set_position(pos)
                    }
                    net_tx.send((MessageKind::Instant, MessageType::GameObjectChangedPosition(name, pos), MsgDst::Boardcast()));
                },
                GameCommand::SetGameObjectRotation(name, rot) => {
                    if let Some(x) = game.gameobjects.get_mut(&name){
                        x.set_rotation_unit(rot)
                    }
                    net_tx.send((MessageKind::Instant, MessageType::GameObjectChangedRotation(name, rot), MsgDst::Boardcast()));
                },
                GameCommand::GetGameObjectPosition(name) => {
                    if let Some(x) = game.gameobjects.get_mut(&name){
                        let channels = LUA_CHANNL_IN.0.lock().unwrap();
                        let _ = channels.send(LuaCommand::ReturnVec(vec![x.position[0], x.position[1], x.position[2]]));
                    }
                    else{
                        let channels = LUA_CHANNL_IN.0.lock().unwrap();
                        let _ = channels.send(LuaCommand::ReturnVec(vec![0.0, 0.0, 0.0]));
                    }
                },
                GameCommand::GetGameObjectRotation(name) => {
                    if let Some(x) = game.gameobjects.get_mut(&name){
                        let channels = LUA_CHANNL_IN.0.lock().unwrap();
                        let rotation = x.rotation.to_euler_angles();
                        let _ = channels.send(LuaCommand::ReturnVec(vec![rotation.0, rotation.1, rotation.2]));
                    }
                    else{
                        let channels = LUA_CHANNL_IN.0.lock().unwrap();
                        let _ = channels.send(LuaCommand::ReturnVec(vec![0.0, 0.0, 0.0]));
                    }
                },
                GameCommand::SendLua(script, id) => {
                    net_tx.send((MessageKind::Reliable, MessageType::LuaScript(script), MsgDst::Id(id)));
                },
                GameCommand::CallEvent(name, args) => {
                    let _ = self.tx.send(LuaLocalCommand::CallEvent(name, args));
                },
                GameCommand::GetObjects() => {
                    let mut objects = vec![];
                    for (name, x) in &game.gameobjects{
                        objects.push(LuaEntity{name: name.clone()});
                    }
                    let channels = LUA_CHANNL_IN.0.lock().unwrap();
                    let _ = channels.send(LuaCommand::GetObjects(objects));
                },
                _ => {

                }
            }
        }
    }
}
