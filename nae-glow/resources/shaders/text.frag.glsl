#version 300 es
precision mediump float;

in vec4 v_color;
out vec4 outColor;

in vec2 v_texcoord;
uniform sampler2D u_texture;

void main() {
    float alpha = texture(u_texture, v_texcoord).r;
    if(alpha <= 0.0) {
        discard;
    }

    outColor = v_color * vec4(1.0, 1.0, 1.0, alpha);
}