#version 450
precision mediump float;
layout(location = 0) in vec2 v_uvs;
layout(binding = 0) uniform sampler2D u_texture;
layout(location = 0) out vec4 color;

#include "pixelize.glsl"
// OR
// #include <examples/pixelize.glsl>

void main() {
    vec2 uv = v_uvs;
    uv = pixelize(uv, 32.);

    color = texture(u_texture, uv);
}
