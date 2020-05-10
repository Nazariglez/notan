#version 450
precision mediump float;

layout(location = 0) out vec4 outColor;
layout(location = 0) in vec2 v_texcoord;

layout(location = 0) uniform sampler2D u_texture;
layout(location = 1) uniform vec2 u_tex_size;
layout(location = 2) uniform vec2 u_size;

void main() {
    vec2 coord = fract(v_texcoord) * u_tex_size;
    coord = floor(coord/u_size) * u_size;
    outColor = texture(u_texture, coord / u_tex_size);
}