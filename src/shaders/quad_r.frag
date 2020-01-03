#version 300 es

precision mediump float;

in vec2 tex;
uniform sampler2D texture_sampler;

out vec4 color;

void main() {
    float c = texture(texture_sampler, tex).r;
    color = vec4(vec3(c), 1.0);
}
