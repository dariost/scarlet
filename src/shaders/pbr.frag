#version 300 es

precision mediump float;

#define MAX_LIGHTS 32

struct Light {
    vec4 position;
    vec3 color;
    float intensity;
};

uniform Light light[MAX_LIGHTS];
uniform uint n_lights;
uniform vec3 camera_pos;

out vec4 color_output;

in vec2 tex_coord;

const float PI = acos(-1.0);

float DistributionGGX(vec3 N, vec3 H, float roughness)
{
    float a      = roughness*roughness;
    float a2     = a*a;
    float NdotH  = max(dot(N, H), 0.0);
    float NdotH2 = NdotH*NdotH;

    float num   = a2;
    float denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;

    return num / denom;
}

float GeometrySchlickGGX(float NdotV, float roughness)
{
    float r = (roughness + 1.0);
    float k = (r*r) / 8.0;

    float num   = NdotV;
    float denom = NdotV * (1.0 - k) + k;

    return num / denom;
}

float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness)
{
    float NdotV = max(dot(N, V), 0.0);
    float NdotL = max(dot(N, L), 0.0);
    float ggx2  = GeometrySchlickGGX(NdotV, roughness);
    float ggx1  = GeometrySchlickGGX(NdotL, roughness);

    return ggx1 * ggx2;
}

vec3 fresnelSchlick(float cosTheta, vec3 F0)
{
    cosTheta = min(cosTheta, 1.0);
    return F0 + (1.0 - F0) * pow(1.0 - cosTheta, 5.0);
}

/*struct Material {
    vec4 albedo;
    float metalness;
    float roughness;
};

in vec3 pos;
in vec3 norm;
uniform Material material;*/

uniform sampler2D position_sampler;
uniform sampler2D normal_sampler;
uniform sampler2D albedo_sampler;
uniform sampler2D metalness_sampler;
uniform sampler2D roughness_sampler;

void main()
{
    vec3 pos = texture(position_sampler, tex_coord).rgb;
    vec3 norm = texture(normal_sampler, tex_coord).rgb;
    vec3 albedo = texture(albedo_sampler, tex_coord).rgb;
    float metalness = texture(metalness_sampler, tex_coord).r;
    float roughness = texture(roughness_sampler, tex_coord).r;
    if(norm == vec3(0.0)) discard;

    vec3 N = normalize(norm);
    vec3 V = normalize(camera_pos - pos);

    vec3 F0 = vec3(0.04);
    F0 = mix(F0, albedo, metalness);

    // reflectance equation
    vec3 Lo = vec3(0.0);
    for(uint i = uint(0); i < n_lights; i++)
    {
        // calculate per-light radiance
        vec3 L;
        if(light[i].position.w != 0.0)
            L = normalize(vec3(light[i].position) - pos);
        else
            L = vec3(light[i].position);
        vec3 H = normalize(V + L);
        float distance    = length(vec3(light[i].position) - pos);
        if(light[i].position.w == 0.0)
            distance = 1.0;
        float attenuation = light[i].intensity / (distance * distance);
        vec3 radiance     = light[i].color * attenuation;

        // cook-torrance brdf
        float NDF = DistributionGGX(N, H, roughness);
        float G   = GeometrySmith(N, V, L, roughness);
        vec3 F    = fresnelSchlick(max(dot(H, V), 0.0), F0);

        vec3 kS = F;
        vec3 kD = vec3(1.0) - kS;
        kD *= 1.0 - metalness;

        vec3 numerator    = NDF * G * F;
        float denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0);
        vec3 specular     = numerator / max(denominator, 0.001);

        // add to outgoing radiance Lo
        float NdotL = max(dot(N, L), 0.0);
        Lo += (kD * albedo / PI + specular) * radiance * NdotL;
    }

    vec3 ambient = vec3(0.03) * albedo;
    vec3 color = ambient + Lo;

    color = color / (color + vec3(1.0));
    color = pow(color, vec3(1.0/2.2));

    color_output = vec4(color, 1.0);
}
