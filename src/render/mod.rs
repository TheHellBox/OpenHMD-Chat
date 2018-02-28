pub mod window;
pub mod draw_display;
pub mod camera;

use glium::{Display};
use glium::vertex::VertexBufferAny;
use std::collections::HashMap;
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2]
}
implement_vertex!(Vertex, position, normal, tex_coords);

pub struct Mesh{
    pub mesh: VertexBufferAny
}

pub struct RenderObject{
    pub id: i32,
    pub mesh_id: i32,
    pub tex_id: i32
}

pub struct RenderData{
    pub mesh_buf: HashMap<String, Mesh>,
    pub render_obj_buf: Vec<RenderObject>,
}

pub const shader_distortion_frag: &'static str = r#"
#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;

void main() {
    color = texture(tex, v_tex_coords);
}
"#;

pub const shader_distortion_vert: &'static str = r#"
#version 140

in vec3 position;
in vec3 normal;
in vec2 tex_coords;
out vec3 v_normal;
out vec2 v_tex_coords;
uniform mat4 perspective;
uniform mat4 matrix;
uniform mat4 view;

void main() {
    mat4 modelview = view * matrix;
    v_normal = transpose(inverse(mat3(modelview))) * normal;
    gl_Position = perspective * modelview * vec4(position, 1.0);
}
"#;
