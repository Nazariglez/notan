#version 450
precision mediump float;

layout(location = 0) in vec2 v_uv;
layout(location = 0) out vec4 color;
uniform sampler2D u_texture;

void main() {
    color = texture(u_texture, v_uv);
//        color = vec4(1.0, 0.0, 0.0, 1.0);
}