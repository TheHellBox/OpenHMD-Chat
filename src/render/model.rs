use render::{Window};
use support::obj_loader;
use glium::texture::Texture2d;
use glium::vertex::VertexBufferAny;

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
}
