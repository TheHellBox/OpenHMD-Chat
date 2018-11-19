use tobj;
use glium::Display;
use render::Vertex;
use render::model::*;
use support::texture_loader;
use glium::vertex::VertexBuffer;

pub fn load(data: String, disp: &Display) -> Model{
    use std::path::Path;

    let raw = tobj::load_obj(&Path::new(&data));
    let (models, materials) = raw.unwrap();
    let mut meshes = vec![];
    for model in &models {
        let mut vertex_data = Vec::new();
        let mesh = &model.mesh;
        let mut material_id = 999;
        if let Some(x) = mesh.material_id{
            material_id = x
        };
        let material: tobj::Material = {
            if material_id != 999{
                materials[material_id].clone()
            }
            else{
                tobj::Material::empty()
            }
        };
        let mut diffuse_texture = material.diffuse_texture;
        for idx in &mesh.indices {
            let i = *idx as usize;
            let pos = [mesh.positions[3 * i], mesh.positions[3 * i + 1], mesh.positions[3 * i + 2]];
            let normal =
                if !mesh.normals.is_empty() {
                    [mesh.normals[3 * i], mesh.normals[3 * i + 1], mesh.normals[3 * i + 2]]
                } else {
                    [0.0, 0.0, 0.0]
            };
            let texcord =
                if !mesh.texcoords.is_empty() {
                    [mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1]]
                } else {
                    [0.0, 0.0]
            };
            vertex_data.push(Vertex {
                position: pos,
                normal: normal,
                tex_coords: texcord
            });
        }
        let texture = texture_loader::load(diffuse_texture, disp);
        let vert_buf = VertexBuffer::new(disp, &vertex_data).unwrap().into_vertex_buffer_any();
        let mesh = Mesh{
            mesh: vert_buf,
            texture
        };
        meshes.push(mesh);
    }
    Model{
        meshes
    }
}
