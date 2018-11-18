use hlua;
use game::GameCommand;
use scripting_engine::{LuaCommand, LUA_CHANNL_IN, LUA_CHANNL_OUT};
use nalgebra::{Translation3, Vector3, Point3};

pub fn get_game_value_raycast(game_cmd: GameCommand) -> RaycastResult{
    {
        let _ = LUA_CHANNL_OUT.0.lock().unwrap().send(game_cmd);
    }
    let data = {
        let data = LUA_CHANNL_IN.1.lock().unwrap().recv();
        data.unwrap_or(LuaCommand::ReturnRaycast(RaycastResult::empty())).clone()
    };
    match data{
        LuaCommand::ReturnRaycast(ray) => {
            ray
        }
        _ => {
            RaycastResult::empty()
        }
    }
}

#[derive(Clone)]
pub struct RaycastResult{
    pub object: String,
    pub position: Point3<f32>
}

impl RaycastResult{
    pub fn empty() -> RaycastResult{
        RaycastResult{
            object: "None".to_string(),
            position: Point3::new(0.0, 0.0, 0.0)
        }
    }
}

implement_lua_read!(RaycastResult);
implement_lua_push!(RaycastResult, |mut metatable| {
    let mut index = metatable.empty_array("__index");
    index.set("Point", hlua::function1(|ent: &mut RaycastResult|{
        vec![ent.position[0], ent.position[1], ent.position[2]]
    }));
});

pub struct RaycastBuilder {
    pub position: Point3<f32>,
    pub direction: Vector3<f32>,
}

impl RaycastBuilder {
    pub fn new() -> RaycastBuilder{
        RaycastBuilder{
            position: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, 0.0)
        }
    }
}
implement_lua_read!(RaycastBuilder);
implement_lua_push!(RaycastBuilder, |mut metatable| {
    let mut index = metatable.empty_array("__index");
    index.set("WithPosition", hlua::function4(|ent: &mut RaycastBuilder, x: f32, y: f32, z: f32|{
        ent.position = Point3::new(x, y, z);
    }));
    index.set("WithDirection", hlua::function4(|ent: &mut RaycastBuilder, x: f32, y: f32, z: f32|{
        ent.direction = Vector3::new(x, y, z);
    }));
    index.set("Build", hlua::function1(|ent: &mut RaycastBuilder|{
        get_game_value_raycast(GameCommand::MakeRaycast(ent.position, ent.direction))
    }));
});
