pub mod window;
pub mod draw_display;
pub mod camera;

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
    pub mesh_name: String,
    pub tex_name: String,
    pub position: (f32, f32, f32),
    pub rotation: (f32, f32, f32)
}

pub struct RenderData{
    pub mesh_buf: HashMap<String, Mesh>,
    pub render_obj_buf: HashMap<u32, RenderObject>,
}

pub const shader_distortion_frag: &'static str = r#"
#version 140

in vec3 v_normal;
out vec4 color;
uniform vec3 u_light;

void main() {
    float brightness = dot(normalize(v_normal), normalize(u_light));
    vec3 dark_color = vec3(0.3, 0.3, 0.3);
    vec3 regular_color = vec3(0.8, 0.8, 0.8);
    color = vec4(mix(dark_color, regular_color, brightness), 1.0);
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
