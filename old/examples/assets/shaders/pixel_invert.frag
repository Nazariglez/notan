#version 450
precision mediump float;

layout(location = 0) out vec4 outColor;
layout(location = 0) in vec2 v_texcoord;

layout(location = 0) uniform sampler2D u_texture;
layout(location = 1) uniform vec2 u_tex_size;
layout(location = 2) uniform float u_value;

void main() {
    vec2 size = vec2(u_value, u_value);
    vec2 coord = fract(v_texcoord) * u_tex_size;
    coord = floor(coord/size) * size;
    vec4 tex_color = texture(u_texture, coord / u_tex_size);

    float red = tex_color.r + ((1.0 - tex_color.r) * abs(sin(u_value)));
    float green = tex_color.g + ((1.0 - tex_color.g) * abs(sin(u_value)));
    float blue = tex_color.b + ((1.0 - tex_color.b) * abs(sin(u_value)));
    outColor = vec4(red, green, blue, tex_color.a);
}