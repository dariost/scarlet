#version 300 es

precision mediump float;

uniform mat4 camera;
uniform mat4 world;

out vec4 color;

void main() {
    color = vec4(1.0, 1.0, 1.0, 1.0);
}
