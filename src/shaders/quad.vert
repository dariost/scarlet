#version 300 es

precision mediump float;

layout(location = 0) in vec2 vert_pos;
layout(location = 1) in vec2 vert_tex;

out vec2 tex;

void main() {
    gl_Position = vec4(vert_pos, 0.0, 1.0);
    tex = vert_tex;
}
