use hlua;
use game::GameCommand;
use scripting_engine::{LUA_CHANNL_OUT, LUA_CHANNL_IN, LuaCommand, get_game_value};

pub struct LuaPlayer{
    pub id: u32
}

implement_lua_read!(LuaPlayer);
implement_lua_push!(LuaPlayer, |mut metatable| {
    let mut index = metatable.empty_array("__index");
    index.set("GetPosition", hlua::function1(|ent: &mut LuaPlayer|{
        get_game_value(GameCommand::GetGameObjectPosition(format!("player{}", ent.id)))
    }));
    index.set("GetRotation", hlua::function1(|ent: &mut LuaPlayer|{
        get_game_value(GameCommand::GetGameObjectRotation(format!("player{}", ent.id)))
    }));
    index.set("SendLua", hlua::function2(|ent: &mut LuaPlayer, script: String|{
        let channels = LUA_CHANNL_OUT.0.lock().unwrap();
        let _ = channels.send(GameCommand::SendLua(script, ent.id));
    }));
});
