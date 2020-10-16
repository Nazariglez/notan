#version 450
precision mediump float;

layout(location = 0) in vec2 v_texcoord;

layout(location = 0) out vec4 outColor;

layout(location = 0) uniform sampler2D u_texture;

void main() {
    outColor = texture(u_texture, v_texcoord);
}