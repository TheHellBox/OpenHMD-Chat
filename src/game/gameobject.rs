use nalgebra::geometry::{Point3, UnitQuaternion, Quaternion};
use scripting_engine;
use render::Window;
use hlua;

#[derive(Clone)]
pub struct GameObject{
    pub name: String,
    pub render_object: Option<String>,
    pub physic_body: Option<u32>,
    pub position: Point3<f32>,
    pub rotation: UnitQuaternion<f32>
}
implement_lua_read!(GameObject);
impl GameObject{
    pub fn new(name: String) -> GameObject{
        GameObject{
            name,
            render_object: None,
            physic_body: None,
            position: Point3::new(0.0, 0.0, 0.0),
            rotation: UnitQuaternion::from_quaternion(Quaternion::new(0.0, 0.0, 0.0, 1.0)),
        }
    }
    pub fn set_render_object(&mut self, name: String){
        self.render_object = Some(name);
    }
    pub fn set_physic_body(&mut self, id: u32){
        self.physic_body = Some(id);
    }
    pub fn set_position(&mut self, pos: Point3<f32>){
        self.position = pos;
    }
    pub fn set_rotation(&mut self, rot: Quaternion<f32>){
        self.rotation = UnitQuaternion::from_quaternion(rot);
    }
    pub fn set_rotation_unit(&mut self, rot: UnitQuaternion<f32>){
        self.rotation = rot;
    }
    pub fn update(&mut self, window: &mut Window){
        if let &Some(ref name) = &self.render_object{
            if let Some(ref mut object) = window.draw_buffer.objects.get_mut(name){
                object.position = self.position;
                object.rotation = self.rotation;
            }
        }
    }
}

#[derive(Clone)]
pub struct GameObjectBuilder{
    pub name: String,
    pub render_object: Option<String>,
    pub physic_body: Option<u32>,
    pub position: Point3<f32>,
    pub rotation: UnitQuaternion<f32>
}
implement_lua_read!(GameObjectBuilder);

impl GameObjectBuilder{
    pub fn new() -> GameObjectBuilder{
        GameObjectBuilder{
            name: "none".to_string(),
            render_object: None,
            physic_body: None,
            position: Point3::new(0.0, 0.0, 0.0),
            rotation: UnitQuaternion::from_quaternion(Quaternion::new(0.0, 0.0, 0.0, 1.0)),
        }
    }
    pub fn with_name(mut self, name: String){
        self.name = name
    }
    pub fn with_position(mut self, pos: Point3<f32>){
        self.position = pos
    }
    pub fn with_rotation(mut self, rot: Quaternion<f32>){
        self.rotation = UnitQuaternion::from_quaternion(rot);
    }
    pub fn with_rotation_unit(mut self, rot: UnitQuaternion<f32>){
        self.rotation = rot;
    }
    pub fn with_render_object(mut self, name: String){
        self.render_object = Some(name);
    }
    pub fn with_physic_body(mut self, id: u32){
        self.physic_body = Some(id);
    }
    pub fn build(self) -> GameObject{
        GameObject{
            name: self.name,
            render_object: self.render_object,
            physic_body: self.physic_body,
            position: self.position,
            rotation: self.rotation
        }
    }
}

implement_lua_push!(GameObjectBuilder, |mut metatable| {
    let mut index = metatable.empty_array("__index");
    index.set("with_position", hlua::function4(|go_builder: &mut GameObjectBuilder, x: f32, y: f32, z: f32| go_builder.position = Point3::new(x, y, z) ));
    index.set("build", hlua::function1(|go_builder: &mut GameObjectBuilder| {
        let game_object = go_builder.clone().build();
        let name = game_object.name.clone();
        let channels = scripting_engine::LUA_CHANNL_IN.lock().unwrap();
        let _ = channels.0.send(scripting_engine::LuaEvent::SpawnGameObject(game_object));
        scripting_engine::LuaEntity{
            name
        }
    } ));
});
