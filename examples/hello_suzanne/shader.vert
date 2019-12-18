#version 300 es

precision mediump float;

#define MAX_LIGHTS 16

struct Light {
    vec4 position;
    vec3 color;
    float intensity;
};

uniform Light light[MAX_LIGHTS];
uniform uint n_lights;
uniform mat4 camera;
uniform mat4 world;

layout(location = 0) in vec3 vertPos;
layout(location = 1) in vec3 vertNorm;

out vec3 norm;

void main() {
    gl_Position = camera * world * vec4(vertPos, 1.0);
    norm = vec3(world * vec4(vertNorm, 0));
}
