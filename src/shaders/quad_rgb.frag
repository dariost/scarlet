#version 300 es

precision mediump float;

in vec2 tex;
uniform sampler2D texture_sampler;

out vec4 color;

void main() {
    color = vec4(texture(texture_sampler, tex).rgb, 1.0);
}
