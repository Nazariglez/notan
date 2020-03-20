#version 450
precision mediump float;

layout(location = 0) in vec4 v_color;
layout(location = 0) out vec4 color;

void main() {
    color = v_color;
}