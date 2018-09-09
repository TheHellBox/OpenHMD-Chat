use render::{Window};
use support::obj_loader;
use glium::texture::Texture2d;
use glium::vertex::VertexBufferAny;
use nalgebra::{Point3, UnitQuaternion};
pub struct Model{
    pub meshes: Vec<Mesh>,
}

pub struct Mesh{
    pub mesh: VertexBufferAny,
    pub texture: Texture2d
}

impl Window{
    pub fn load_model(&self, path: String) -> Model{
        obj_loader::load(path.to_string(), &self.display)
    }
    pub fn load_model_and_push(&mut self, path: String, name: String, size: (f32, f32, f32)){
        let model = obj_loader::load(path.to_string(), &self.display);
        self.add_draw_object(name, model,
            Point3::new(0.0, 0.0, 0.0),
            UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
            (size.0, size.1, size.2),
            "simple");
    }
}
