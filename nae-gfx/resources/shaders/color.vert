#version 450

layout(location = 0) in vec4 a_position;
layout(location = 1) in vec4 a_color;

layout(location = 0) out vec4 v_color;
layout(location = 0) uniform mat4 u_matrix;

void main() {
    v_color = a_color;
//    gl_Position = vec4((u_matrix * vec3(a_position, 1)).xy - 0.5, 0.0, 1.0);
    gl_Position = u_matrix * a_position;
}