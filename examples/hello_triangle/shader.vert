#version 300 es

precision mediump float;

layout(location = 0) in vec3 vertPos;
layout(location = 1) in vec3 vertColor;

out vec3 fragColor;

void main() {
    fragColor = vertColor;
    gl_Position = vec4(vertPos, 1.0);
}
