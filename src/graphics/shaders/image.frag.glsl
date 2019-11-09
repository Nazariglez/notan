#version 300 es
precision mediump float;

in vec4 v_color;
out vec4 outColor;

in vec2 v_texcoord;
uniform sampler2D u_texture;

void main() {
    outColor = texture(u_texture, v_texcoord) * v_color;
}