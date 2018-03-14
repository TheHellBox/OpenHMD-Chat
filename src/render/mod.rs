pub mod window;
pub mod draw_display;
pub mod camera;

use glium::vertex::VertexBufferAny;
use glium::Texture2d;
use std::collections::HashMap;
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2]
}

#[derive(Copy, Clone)]
pub struct OhmdVertex {
    pub coords: [f32; 2],
}
implement_vertex!(Vertex, position, normal, tex_coords);
implement_vertex!(OhmdVertex, coords);

pub struct Mesh{
    pub mesh: VertexBufferAny
}

pub struct RenderObject{
    pub mesh_name: String,
    pub tex_name: String,
    pub position: (f32, f32, f32),
    pub rotation: (f32, f32, f32, f32),
    pub size: (f32, f32, f32),
    pub visible: bool,
}

pub struct RenderData{
    pub mesh_buf: HashMap<String, Mesh>,
    pub texture_buf: HashMap<String, Texture2d>,
    pub render_obj_buf: HashMap<u32, RenderObject>,
}

pub struct HMDParams{
    pub scr_size_w: f32,
    pub scr_size_h: f32,
    pub left_lens_center: [f32; 2],
    pub right_lens_center: [f32; 2],
    pub view_port_scale: [f32; 2],
    pub distortion_k: [f32; 4],
    pub aberration_k: [f32; 3]
}

pub const shader_simple_frag: &'static str = r#"
#version 140
in vec3 v_normal;
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;
void main() {
    vec3 dark_color = vec3(0.3, 0.3, 0.3);
    vec3 regular_color = vec3(0.8, 0.8, 0.8);
    color = texture(tex, v_tex_coords);
}
"#;

pub const shader_simple_vert: &'static str = r#"
#version 330

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
    v_tex_coords = tex_coords;
}
"#;

pub const shader_distortion_frag: &'static str = r#"
#version 330
uniform sampler2D warpTexture;
uniform vec2 LensCenter;
uniform vec2 ViewportScale;
uniform float WarpScale;
uniform vec4 HmdWarpParam;
uniform vec3 aberr;
in vec2 T;
in vec2 v_tex_coords;
out vec4 color;

void main()
{
    vec2 output_loc = vec2(T.s, T.t);
    vec2 r = output_loc * ViewportScale - LensCenter;
    r /= WarpScale;

    float r_mag = length(r);
    vec2 r_displaced = r * (HmdWarpParam.w + HmdWarpParam.z * r_mag +
    HmdWarpParam.y * r_mag * r_mag +
    HmdWarpParam.x * r_mag * r_mag * r_mag);
    r_displaced *= WarpScale;
    vec2 tc_r = (LensCenter + aberr.r * r_displaced) / ViewportScale;
    vec2 tc_g = (LensCenter + aberr.g * r_displaced) / ViewportScale;
    vec2 tc_b = (LensCenter + aberr.b * r_displaced) / ViewportScale;
    float red = texture(warpTexture, tc_r).r;
    float green = texture(warpTexture, tc_g).g;
    float blue = texture(warpTexture, tc_b).b;

    color =  ((tc_g.x < 0.0) || (tc_g.x > 1.0) || (tc_g.y < 0.0) || (tc_g.y > 1.0)) ? vec4(0.0, 0.0, 0.0, 1.0) : vec4(red, green, blue, 1.0);
}
"#;

pub const shader_distortion_vert: &'static str = r#"
#version 330

in vec2 coords;
uniform mat4 mvp;
out vec2 T;

void main(void) {
    T = coords;
    gl_Position = mvp * vec4(coords, 0.0, 1.0);
}
"#;
