#version 300 es
in vec2 a_position;
in vec4 a_color;
out vec4 v_color;

in vec2 a_texcoord;
out vec2 v_texcoord;
out float v_scale;

uniform mat3 u_matrix;

void main() {
    v_scale = 5.0;
    vec2 coord = a_texcoord;
//    vec2 size = vec2(64.0, 64.0) * v_scale;
//    vec2 offset = fract((size - vec2(0.0, 0.0)) / size);
//    coord += offset;
    v_color = a_color;
    v_texcoord = coord+1.2;
    gl_Position = vec4((u_matrix * vec3(a_position, 1)).xy, 0, 1);
}