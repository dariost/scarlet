#version 300 es

precision mediump float;

uniform mat4 camera;
uniform mat4 world;

out vec4 color;

in vec3 norm;

void main() {
    color = vec4(norm, 1.0);
}
