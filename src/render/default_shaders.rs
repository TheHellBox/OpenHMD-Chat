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
    color.a = 1.0;
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
