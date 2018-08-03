use nalgebra::geometry::{Point3, UnitQuaternion, Translation3, Quaternion};
use std::borrow::Borrow;
use render::Window;

pub struct GameObject{
    pub name: String,
    pub render_object: Option<String>,
    pub physic_body: Option<u32>,
    pub position: Point3<f32>,
    pub rotation: UnitQuaternion<f32>
}
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


pub struct GameObjectBuilder{
    pub name: String,
    pub render_object: Option<String>,
    pub physic_body: Option<u32>,
    pub position: Point3<f32>,
    pub rotation: UnitQuaternion<f32>
}

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
