pub extern crate tobj;

use glium;
use glium::Display;

use render::Vertex;
use render::Mesh;

use std::collections::HashMap;

pub fn load(data: String) -> Vec<Vertex> {
    use std::path::Path;
    let raw = tobj::load_obj(&Path::new(&data));
    assert!(raw.is_ok());
    let (models, _) = raw.unwrap();
    let mut vertex_data = Vec::new();

    for model in &models {
        let mesh = &model.mesh;
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

pub fn gen_buffer(disp: &Display) -> HashMap<String, Mesh>{
    use std::fs;
    let paths = fs::read_dir("./assets/models/").unwrap();
    let mut models: HashMap<String, Mesh> = HashMap::with_capacity(1024);
    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            let name = path.display().to_string();
            if path.extension().unwrap() == "obj"{
                print!("Loading model {} ... ", path.display());
                let raw = load(path.display().to_string());
                let mesh = glium::vertex::VertexBuffer::new(disp, &raw).unwrap().into_vertex_buffer_any();

                models.insert(name, Mesh{mesh: mesh});

                println!("Done!");
            }
        }
    }
    models
}
