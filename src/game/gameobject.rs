use nalgebra::geometry::{Point3, UnitQuaternion, Quaternion, Translation3};
use nalgebra::core::{Matrix4};
use scripting_engine;
use render::Window;
use hlua;

#[derive(Clone)]
pub struct GameObject{
    pub name: String,
    pub render_object: String,
    pub physic_body: u32,
    pub position: Point3<f32>,
    pub rotation: UnitQuaternion<f32>,
    pub scale: (f32, f32, f32)
}
implement_lua_read!(GameObject);
impl GameObject{
    pub fn new(name: String) -> GameObject{
        GameObject{
            name,
            render_object: "none".to_string(),
            physic_body: 0,
            position: Point3::new(0.0, 0.0, 0.0),
            rotation: UnitQuaternion::from_quaternion(Quaternion::new(0.707, 0.0, 0.707, 0.0)),
            scale: (0.1, 0.1, 0.1)
        }
    }
    pub fn set_render_object(&mut self, name: String){
        self.render_object = name;
    }
    pub fn set_physic_body(&mut self, id: u32){
        self.physic_body = id;
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
    pub fn calc_transform(&self) -> Matrix4<f32>{
        let scale_matrix: Matrix4<f32> = Matrix4::new(
            self.scale.0 as f32, 0.0, 0.0, 0.0,
            0.0, self.scale.1 as f32, 0.0, 0.0,
            0.0, 0.0, self.scale.2 as f32, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );
        let translation_matrix = Translation3::from_vector(self.position.coords).to_homogeneous();
        let rotation_matrix = self.rotation.to_homogeneous();
        translation_matrix * rotation_matrix * scale_matrix
    }
    pub fn update(&mut self, window: &mut Window){
        let name = self.render_object.clone();
        if name != "none".to_string(){
            if let Some(ref mut object) = window.draw_buffer.objects.get_mut(&name){
                object.position = self.position;
                object.rotation = self.rotation;
            }
        }
    }
}

#[derive(Clone)]
pub struct GameObjectBuilder{
    pub name: String,
    pub render_object: String,
    pub physic_body: u32,
    pub position: Point3<f32>,
    pub rotation: UnitQuaternion<f32>
}
implement_lua_read!(GameObjectBuilder);

impl GameObjectBuilder{
    pub fn new() -> GameObjectBuilder{
        GameObjectBuilder{
            name: "none".to_string(),
            render_object: "none".to_string(),
            physic_body: 0,
            position: Point3::new(0.0, 0.0, 0.0),
            rotation: UnitQuaternion::from_quaternion(Quaternion::new(0.0, 0.0, 0.0, 1.0)),
        }
    }
    pub fn with_name(mut self, name: String) -> Self{
        GameObjectBuilder{
            name,
            ..self
        }
    }
    pub fn with_position(mut self, position: Point3<f32>) -> Self{
        GameObjectBuilder{
            position,
            ..self
        }
    }
    pub fn with_rotation(mut self, rotation: Quaternion<f32>) -> Self{
        let rotation = UnitQuaternion::from_quaternion(rotation);
        GameObjectBuilder{
            rotation,
            ..self
        }
    }
    pub fn with_rotation_unit(mut self, rotation: UnitQuaternion<f32>) -> Self{
        GameObjectBuilder{
            rotation,
            ..self
        }
    }
    pub fn with_render_object(mut self, render_object: String) -> Self{
        GameObjectBuilder{
            render_object,
            ..self
        }
    }
    pub fn with_physic_body(mut self, physic_body: u32) -> Self{
        GameObjectBuilder{
            physic_body,
            ..self
        }
    }
    pub fn build(self) -> GameObject{
        GameObject{
            name: self.name,
            render_object: self.render_object,
            physic_body: self.physic_body,
            position: self.position,
            rotation: self.rotation,
            scale: (0.1, 0.1, 0.1)
        }
    }
}

implement_lua_push!(GameObjectBuilder, |mut metatable| {
    let mut index = metatable.empty_array("__index");
    index.set("with_position", hlua::function4(|go_builder: &mut GameObjectBuilder, x: f32, y: f32, z: f32| go_builder.position = Point3::new(x, y, z) ));
    index.set("with_model", hlua::function2(|go_builder: &mut GameObjectBuilder, name: String| go_builder.render_object = name ));
    index.set("build", hlua::function1(|go_builder: &mut GameObjectBuilder| {
        let game_object = go_builder.clone().build();
        let name = game_object.name.clone();
        let channels = scripting_engine::LUA_CHANNL_OUT.0.lock().unwrap();
        let _ = channels.send(scripting_engine::LuaEvent::SpawnGameObject(game_object));
        scripting_engine::LuaEntity{
            name
        }
    } ));
});
