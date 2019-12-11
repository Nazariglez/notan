#version 300 es
precision mediump float;

out vec4 outColor;
in vec2 v_texcoord;
uniform sampler2D u_texture;

uniform vec2 u_tex_size;
uniform vec2 u_size;

void main() {
    vec2 coord = fract(v_texcoord) * u_tex_size;
    coord = floor(coord/u_size) * u_size;
    outColor = texture(u_texture, coord / u_tex_size);
}