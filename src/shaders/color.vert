#version 450

layout(location = 0) in vec4 a_position;
layout(location = 1) in vec4 a_color;

layout(location = 0) out vec4 v_color;
//layout(location = 0) uniform mat4 u_matrix;

void main() {
    v_color = a_color;
//    gl_Position = u_matrix * a_position;
    gl_Position = a_position;
}