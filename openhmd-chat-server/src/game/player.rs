use hlua;
use game::GameCommand;
use nalgebra::{Point3, UnitQuaternion, Vector3};
use scripting_engine::{LUA_CHANNL_OUT, get_game_value};

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
    index.set("Direction", hlua::function4(|ent: &mut LuaPlayer, x: f32, y: f32, z: f32|
        {
            use support::direction;
            let rot = get_game_value(GameCommand::GetGameObjectRotation(format!("player{}", ent.id)));
            let rot = UnitQuaternion::from_euler_angles(rot[0], rot[1], rot[2]);
            let dir = direction(rot, Vector3::new(x, y, z));
            vec![dir[0], dir[1], dir[2]]
        }
    ));
    index.set("SetPosition", hlua::function4(|ent: &mut LuaPlayer, x: f32, y: f32, z: f32|{
        let channels = LUA_CHANNL_OUT.0.lock().unwrap();
        let _ = channels.send(GameCommand::ChangePlayersCameraPosition(ent.id, Point3::new(x, y, z)));
        let _ = channels.send(GameCommand::SetGameObjectPosition(format!("player{}", ent.id), Point3::new(x, y, z)));
    }));
    index.set("SetRotation", hlua::function4(|ent: &mut LuaPlayer, x: f32, y: f32, z: f32|{
        let rotation = UnitQuaternion::from_euler_angles(x, y, z);
        let channels = LUA_CHANNL_OUT.0.lock().unwrap();
        let _ = channels.send(GameCommand::ChangePlayersCameraRotation(ent.id, rotation));
        let _ = channels.send(GameCommand::SetGameObjectRotation(format!("player{}", ent.id), rotation));
    }));
    index.set("Id", hlua::function1(|ent: &mut LuaPlayer|{
        ent.id
    }));
    index.set("SendLua", hlua::function2(|ent: &mut LuaPlayer, script: String|{
        let channels = LUA_CHANNL_OUT.0.lock().unwrap();
        let _ = channels.send(GameCommand::SendLua(script, ent.id));
    }));
});
