#version 450

layout(location = 0) in vec4 a_position;
layout(location = 1) in vec2 a_uv;
layout(location = 0) out vec2 v_uv;

layout(location = 0) uniform mat4 u_matrix;

void main() {
    v_uv = a_uv;
    gl_Position = u_matrix * a_position;
}