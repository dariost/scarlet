#version 300 es

precision mediump float;

out vec4 color_output;

in vec2 tex_coord;

uniform sampler2D ssr_sampler;
uniform sampler2D metalness_sampler;
uniform sampler2D pbr_sampler;

void main() {
    float metalness = texture(metalness_sampler, tex_coord).r;
    vec3 pbr = texture(pbr_sampler, tex_coord).rgb;
    vec3 ssr = texture(ssr_sampler, tex_coord).rgb;
    color_output = vec4(mix(pbr, ssr, pow(metalness, 2.0) / 2.0), 1.0);
}
