#version 450

layout(location = 0) in vec2 a_position;
layout(location = 1) in vec4 a_color;
layout(location = 0) out vec4 v_color;

layout(set = 0, binding = 0) uniform Locals {
    mat3 u_matrix;
};

void main() {
    v_color = a_color;
    gl_Position = vec4((u_matrix * vec3(a_position, 1)).xy, 0, 1);
}