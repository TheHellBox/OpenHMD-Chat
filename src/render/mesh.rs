use glium::texture::Texture2d;
use glium::vertex::VertexBufferAny;

pub struct Model{
    pub meshes: Vec<Mesh>
}

pub struct Mesh{
    pub mesh: VertexBufferAny,
    pub texture: Texture2d
}
