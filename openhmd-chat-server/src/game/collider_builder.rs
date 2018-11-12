use hlua;
use game::GameCommand;
use ncollide3d::shape::{Cuboid, ShapeHandle};
use nphysics3d::object::{BodyHandle};
use nalgebra::{Vector3};
use scripting_engine::{LUA_CHANNL_OUT, LUA_CHANNL_IN, LuaCommand};

#[derive(Clone)]
pub struct LuaCollider {
    pub handle: Option<BodyHandle>
}
implement_lua_read!(LuaCollider);
implement_lua_push!(LuaCollider, |mut _metatable| {
    //let mut index = metatable.empty_array("__index");
});

pub struct ColliderBuilder {
    pub shape: String,
    pub is_static: bool,
    pub size: Vector3<f32>
}

impl ColliderBuilder{
    pub fn new() -> ColliderBuilder{
        ColliderBuilder{
            shape: "Cube".to_string(),
            is_static: true,
            size: Vector3::repeat(0.1)
        }
    }
}

implement_lua_read!(ColliderBuilder);
implement_lua_push!(ColliderBuilder, |mut metatable| {
    let mut index = metatable.empty_array("__index");
    index.set("WithShape", hlua::function2(|ent: &mut ColliderBuilder, shape: String|{
        ent.shape = shape;
    }));
    index.set("Static", hlua::function2(|ent: &mut ColliderBuilder, is_static: bool|{
        ent.is_static = is_static;
    }));
    index.set("WithSize", hlua::function4(|ent: &mut ColliderBuilder, x: f32, y: f32, z: f32|{
        ent.size = Vector3::new(x, y, z);
    }));
    index.set("Build", hlua::function1(|ent: &mut ColliderBuilder|{
        let geom = ShapeHandle::new(Cuboid::new(ent.size));
        {
            let channels = LUA_CHANNL_OUT.0.lock().unwrap();
            let _ = channels.send(GameCommand::CreateRigidBody(geom, ent.is_static));
        }
        let channels = LUA_CHANNL_IN.1.lock().unwrap();
        let data = channels.recv();
        if data.is_ok(){
            let data = data.unwrap();
            match data{
                LuaCommand::ReturnRBhandler(handle) => {
                    LuaCollider{
                        handle: Some(handle)
                    }
                }
                _ => {
                    LuaCollider{
                        handle: None
                    }
                }
            }
        }
        else{
            LuaCollider{
                handle: None
            }
        }
    }));
});
