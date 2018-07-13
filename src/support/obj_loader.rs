use tobj;
use render::Vertex;

pub fn load(data: String) -> Vec<Vertex> {
    use std::path::Path;
    let raw = tobj::load_obj(&Path::new(&data));
    let (models, materials) = raw.unwrap();
    let mut vertex_data = Vec::new();
    for x in materials{
        println!("{?}", x.diffuse_texture);
    }
    for model in &models {
        let mesh = &model.mesh;
        let material = mesh.material_id;
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
    }
    vertex_data
}
