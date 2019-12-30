#version 300 es

precision mediump float;

uniform mat4 world;
uniform mat4 camera;

layout(location = 0) in vec3 vert_pos;
layout(location = 1) in vec3 vert_norm;

out vec3 pos;
out vec3 norm;

void main() {
    gl_Position = camera * world * vec4(vert_pos, 1.0);
    vec4 posT = world * vec4(vert_pos, 1.0);
    pos = vec3(posT) / posT.w;
    norm = vec3(world * vec4(vert_norm, 0));
}
