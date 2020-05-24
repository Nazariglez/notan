#version 450

layout(location = 0) in vec4 a_position;
layout(location = 1) in vec4 a_color;
layout(location = 2) in vec2 a_texcoord;
layout(location = 3) in vec4 a_frame;

layout(location = 0) out vec4 v_color;
layout(location = 1) out vec2 v_texcoord;
layout(location = 2) out vec4 v_frame;

layout(location = 0) uniform mat4 u_matrix;

void main() {
    v_frame = a_frame;
    v_color = a_color;
    v_texcoord = a_texcoord;
    gl_Position = u_matrix * a_position;
}