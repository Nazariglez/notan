#version 450
precision mediump float;

layout(location = 0) in vec4 v_color;
layout(location = 1) in vec2 v_texcoord;

layout(location = 0) out vec4 outColor;

layout(location = 0) uniform sampler2D u_texture;
layout(location = 1) uniform vec4 u_frame;

void main() {
    vec2 coords = u_frame.xy + fract(v_texcoord) * u_frame.zw;
    outColor = texture(u_texture, coords) * v_color;
}