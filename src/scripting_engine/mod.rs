use hlua;
use game::Game;
use hlua::Lua;
use std::fs::File;
use std::path::Path;
use std::{thread, time};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};
use game::gameobject::{GameObjectBuilder, GameObject};

lazy_static! {
    pub static ref LUA_CHANNL_IN: Arc<Mutex<(Sender<LuaEvent>, Receiver<LuaEvent>)>> = Arc::new(Mutex::new(channel::<LuaEvent>()));
}

pub enum LuaEvent{
    SetGameObjectPosition(String, (f32, f32, f32)),
    SpawnGameObject(GameObject)
}

pub struct LuaEntity{
    pub name: String
}
implement_lua_read!(LuaEntity);
implement_lua_push!(LuaEntity, |mut metatable| {
    let mut index = metatable.empty_array("__index");
    index.set("set_position", hlua::function4(|ent: &mut LuaEntity, x: f32, y: f32, z: f32|
        {
            let channels = LUA_CHANNL_IN.lock().unwrap();
            let _ = channels.0.send(LuaEvent::SetGameObjectPosition(ent.name.clone(), (x, y, z)));
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

        let mut lua = Lua::new();
        //init
        lua.openlibs();
        {
            let mut world = lua.empty_array("World");
            world.set("create_game_object", hlua::function0(|| GameObjectBuilder::new() ));
        }
        thread::spawn(move || {
            lua.execute_from_reader::<(), _>(File::open(&Path::new("./assets/lua/test.lua")).unwrap());
        });
        ScriptingEngine{
            rx
        }
    }
    pub fn update(&mut self, game: &mut Game){
        use nalgebra::Point3;
        let channels = LUA_CHANNL_IN.lock().unwrap();
        let data = channels.1.try_iter();
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
                _ => {}
            }
        }
    }
}
