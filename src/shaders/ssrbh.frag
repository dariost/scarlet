#version 300 es

precision mediump float;

#define MAX_WINDOW_SIZE 63
#define KERNEL_SIZE (WINDOW_SIZE / 2)

uniform sampler2D ssr_sampler;
uniform sampler2D roughness_sampler;
uniform vec2 resolution;

out vec4 color_output;

in vec2 tex_coord;

float gauss(float x, float sigma) {
	return exp(-0.5 * x * x / (sigma * sigma)) / sqrt(2.0 * acos(-1.0) * sigma);
}

void main() {
    float roughness = texture(roughness_sampler, tex_coord).r;
    float sigma = pow(roughness + 1.0, 5.0);
    vec3 ssr = texture(ssr_sampler, tex_coord).rgb;
    color_output = vec4(ssr, 1.0);
    float kernel[MAX_WINDOW_SIZE];
    const int shrink_factor = 64;
    int WINDOW_SIZE = min(MAX_WINDOW_SIZE, int(resolution.x) / shrink_factor + (int(resolution.x) / shrink_factor) % 2);
    for(int i = 0; i <= KERNEL_SIZE; i++) {
        kernel[KERNEL_SIZE - i] = kernel[KERNEL_SIZE + i] = gauss(float(i), sigma);
    }
    float norm_factor = 0.0;
    for(int i = 0; i < WINDOW_SIZE; i++) {
        norm_factor += kernel[i];
    }
    vec3 acc = vec3(0.0);
    for(int i = -KERNEL_SIZE; i <= KERNEL_SIZE; i++) {
        acc += kernel[KERNEL_SIZE + i] * texture(ssr_sampler, tex_coord + (vec2(float(i), 0.0) / resolution)).rgb;
    }
    color_output = vec4(acc / norm_factor, 1.0);
}
