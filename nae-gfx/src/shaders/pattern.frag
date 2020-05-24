#version 450
precision mediump float;

layout(location = 0) in vec4 v_color;
layout(location = 1) in vec2 v_texcoord;
layout(location = 2) in vec4 v_frame;

layout(location = 0) out vec4 outColor;

layout(location = 0) uniform sampler2D u_texture;

void main() {
    vec2 coords = v_frame.xy + fract(v_texcoord) * v_frame.zw;
    outColor = texture(u_texture, coords) * v_color;
}