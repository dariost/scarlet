#version 300 es

precision mediump float;

in vec3 pos;
in vec3 norm;

layout (location = 0) out vec3 g_position;
layout (location = 1) out vec3 g_normal;
layout (location = 2) out vec3 g_albedo;
layout (location = 3) out float g_metalness;
layout (location = 4) out float g_roughness;

struct Material {
    vec4 albedo;
    float metalness;
    float roughness;
};

uniform Material material;

void main() {
    g_position = pos;
    g_normal = normalize(norm);
    g_albedo = material.albedo.rgb;
    g_metalness = material.metalness;
    g_roughness = material.roughness;
}
