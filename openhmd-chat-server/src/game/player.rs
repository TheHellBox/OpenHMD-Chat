use hlua;
use game::GameCommand;
use scripting_engine::{LUA_CHANNL_OUT, LUA_CHANNL_IN, LuaCommand};

pub struct LuaPlayer{
    pub id: u32
}

implement_lua_read!(LuaPlayer);
implement_lua_push!(LuaPlayer, |mut metatable| {
    let mut index = metatable.empty_array("__index");
    index.set("get_position", hlua::function1(|ent: &mut LuaPlayer|{
        {
            let channels = LUA_CHANNL_OUT.0.lock().unwrap();
            let res = channels.send(GameCommand::GetGameObjectPosition(format!("player{}", ent.id)));
        }
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
    }));
    index.set("SendLua", hlua::function2(|ent: &mut LuaPlayer, script: String|{
        let channels = LUA_CHANNL_OUT.0.lock().unwrap();
        let _ = channels.send(GameCommand::SendLua(script, ent.id));
    }));
});
