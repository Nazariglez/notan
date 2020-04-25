#version 450

layout(location = 0) in vec4 a_position;
layout(location = 1) in vec2 a_texcoord;

layout(location = 0) out vec2 v_texcoord;

void main() {
    v_texcoord = a_texcoord;
    gl_Position = a_position;
}