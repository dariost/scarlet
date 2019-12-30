#version 300 es

precision mediump float;

in vec2 tex;
uniform sampler2D texture_sampler;

out vec3 color;

void main() {
    color = texture(texture_sampler, tex).rgb;
}
