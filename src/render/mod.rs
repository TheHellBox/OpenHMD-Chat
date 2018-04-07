pub mod window;
pub mod draw_display;
pub mod camera;

use glium::vertex::VertexBufferAny;
use glium::Texture2d;
use std::collections::HashMap;
use support;
use glium;

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
    pub scale: (f32, f32, f32),
    pub visible: bool,
}

pub struct RenderData{
    pub mesh_buf: HashMap<String, Mesh>,
    pub texture_buf: HashMap<String, Texture2d>,
    pub render_obj_buf: HashMap<u32, RenderObject>,
}

impl RenderData {
    pub fn append_file(&mut self, filename: String, disp: &glium::Display){
        use std::path::Path;
        let path = Path::new(&filename);
        if path.is_file() {
            let name = path.display().to_string();
            if path.extension().unwrap() == "obj"{
                print!("Loading model {} ... ", path.display());
                let raw = support::obj_model_loader::load(path.display().to_string());
                let mesh = glium::vertex::VertexBuffer::new(disp, &raw).unwrap().into_vertex_buffer_any();

                self.mesh_buf.insert((&filename).to_owned(), Mesh{mesh: mesh});

                println!("Done!");
            }
        }
    }
}
pub const SHADER_SIMPLE_FRAG: &'static str = r#"
#version 140
in vec3 v_normal;
in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;

void main() {
    vec3 u_light = vec3(0.1,0.1,0.4);
    float brightness = dot(normalize(v_normal), normalize(u_light));
    vec3 dark_color = vec3(0.7, 0.7, 0.7) * vec3(texture(tex, v_tex_coords));
    vec3 regular_color = vec3(1.0, 1.0, 1.0) * vec3(texture(tex, v_tex_coords));
    color = vec4(mix(dark_color, regular_color, brightness), 1.0);
}
"#;

pub const SHADER_SIMPLE_VERT: &'static str = r#"
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
    v_normal = normal;
    gl_Position = perspective * modelview * vec4(position, 1.0);
    v_tex_coords = tex_coords;
}
"#;

pub const SHADER_DISTORTION_FRAG: &'static str = r#"
#version 330

//per eye texture to warp for lens distortion
uniform sampler2D warpTexture;

//Position of lens center in m (usually eye_w/2, eye_h/2)
uniform vec2 LensCenter;
//Scale from texture co-ords to m (usually eye_w, eye_h)
uniform vec2 ViewportScale;
//Distortion overall scale in m (usually ~eye_w/2)
uniform float WarpScale;
//Distoriton coefficients (PanoTools model) [a,b,c,d]
uniform vec4 HmdWarpParam;

//chromatic distortion post scaling
uniform vec3 aberr;

in vec2 T;
out vec4 color;

void main()
{
    //output_loc is the fragment location on screen from [0,1]x[0,1]
    vec2 output_loc = vec2(T.s, T.t);
    //Compute fragment location in lens-centered co-ordinates at world scale
    vec2 r = output_loc * ViewportScale - LensCenter;
    //scale for distortion model
    //distortion model has r=1 being the largest circle inscribed (e.g. eye_w/2)
    r /= WarpScale;

    //|r|**2
    float r_mag = length(r);
    //offset for which fragment is sourced
    vec2 r_displaced = r * (HmdWarpParam.w + HmdWarpParam.z * r_mag +
    HmdWarpParam.y * r_mag * r_mag +
    HmdWarpParam.x * r_mag * r_mag * r_mag);
    //back to world scale
    r_displaced *= WarpScale;
    //back to viewport co-ord
    vec2 tc_r = (LensCenter + aberr.r * r_displaced) / ViewportScale;
    vec2 tc_g = (LensCenter + aberr.g * r_displaced) / ViewportScale;
    vec2 tc_b = (LensCenter + aberr.b * r_displaced) / ViewportScale;

    float red = texture(warpTexture, tc_r).r;
    float green = texture(warpTexture, tc_g).g;
    float blue = texture(warpTexture, tc_b).b;
    //Black edges off the texture
    color = ((tc_g.x < 0.0) || (tc_g.x > 1.0) || (tc_g.y < 0.0) || (tc_g.y > 1.0)) ? vec4(0.0, 0.0, 0.0, 1.0) : vec4(red, green, blue, 1.0);
}
"#;

pub const SHADER_DISTORTION_VERT: &'static str = r#"
#version 330
layout (location=0) in vec2 coords;
uniform mat4 mvp;
out vec2 T;
void main(void)
{
    T = coords;
    gl_Position = mvp * vec4(coords, 0.0, 1.0);
}
"#;
