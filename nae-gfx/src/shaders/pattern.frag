#version 450
precision mediump float;

layout(location = 0) in vec4 v_color;
layout(location = 1) in vec2 v_texcoord;

layout(location = 0) out vec4 outColor;

layout(location = 0) uniform sampler2D u_texture;
layout(location = 1) uniform vec4 u_frame;


//https://github.com/pixijs/pixi.js/blob/dev/packages/sprite-tiling/src/tilingSprite.frag
//https://community.khronos.org/t/repeat-tile-from-texture-atlas/104500/2
//http://www.java-gaming.org/index.php/topic,28147
void main() {
    vec2 pos = u_frame.xy;
    vec2 size = u_frame.zw;
    vec2 coords = pos + fract(v_texcoord) * size;
    outColor = texture(u_texture, coords) * v_color;
}