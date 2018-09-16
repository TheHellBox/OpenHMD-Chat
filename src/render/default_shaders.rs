pub const SHADER_SIMPLE_FRAG: &'static str = r#"
#version 140
in vec3 v_normal;
in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;
uniform vec2 wrap;

void main() {
    vec3 u_light = vec3(0.1,0.1,0.4);
    float brightness = dot(normalize(v_normal), normalize(u_light));
    vec3 dark_color = vec3(0.7, 0.7, 0.7) * vec3(texture(tex, v_tex_coords + wrap));
    vec3 regular_color = vec3(1.0, 1.0, 1.0) * vec3(texture(tex, v_tex_coords + wrap));
    color = vec4(mix(dark_color, regular_color, brightness), 1.0);
}
"#;

pub const SHADER_SOLID_FRAG: &'static str = r#"
#version 140
in vec3 v_normal;
in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;

void main() {
    color = vec4(texture(tex, v_tex_coords));
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

pub const SHADER_SIMPLE_2D_VERT: &'static str = r#"
#version 330

in vec2 position;
in vec3 normal;
out vec3 v_normal;
out vec2 v_tex_coords;
uniform mat4 matrix;
void main() {
    v_normal = normal;
    gl_Position = matrix * vec4(position, 0.0, 1.0);
    v_tex_coords = position;
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
layout (location=0) in vec2 position;
uniform mat4 mvp;
out vec2 T;
void main(void)
{
    T = position;
    gl_Position = mvp * vec4(position, 0.0, 1.0);
}
"#;
