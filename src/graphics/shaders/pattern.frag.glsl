#version 300 es
precision mediump float;

in vec4 v_color;
out vec4 outColor;

in vec2 v_texcoord;
uniform sampler2D u_texture;

void main() {
    vec2 coord = fract(v_texcoord); //fract(v_texcoord*3.0);
    outColor = texture(u_texture, coord) * v_color;
}