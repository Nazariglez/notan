#version 300 es
precision mediump float;

in vec4 v_color;
out vec4 outColor;

in vec2 v_texcoord;
uniform sampler2D u_texture;

in float v_scale;

void main() {
    vec2 coord = v_texcoord;
    coord = fract(coord*v_scale);
    outColor = texture(u_texture, coord) * v_color;
}