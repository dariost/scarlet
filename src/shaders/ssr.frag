#version 300 es

precision mediump float;

#define N_RAYS 1

uniform vec3 camera_pos;
uniform mat4 camera;
uniform sampler2D position_sampler;
uniform sampler2D normal_sampler;
uniform sampler2D pbr_sampler;
uniform sampler2D metalness_sampler;
uniform sampler2D roughness_sampler;
uniform sampler2D depth_sampler;

in vec2 tex_coord;
in vec2 tex_pos;

out vec4 color_output;

float rand(vec2 co) {
    float a = 12.9898;
    float b = 78.233;
    float c = 43758.5453;
    float dt = dot(co.xy, vec2(a,b));
    float sn = mod(dt, 3.141592);
    return fract(sin(sn) * c);
}

vec3 ray_march(vec3 pos, vec3 dir, float step_size, float rough_factor) {
    float steps = 0.0;
    while(true) {
        pos += dir * step_size;
        vec4 ray_view_homo = camera * vec4(pos, 1.0);
        vec3 view = ray_view_homo.xyz / ray_view_homo.w;
        if(!(abs(view.x) <= 1.0 && abs(view.y) <= 1.0 && abs(view.z) <= 1.0)) {
            break;
        }
        vec2 coord = vec2(view.x + 1.0, view.y + 1.0) / 2.0;
        float depth = texture(depth_sampler, coord).r;
        if(view.z >= depth) {
            return textureLod(pbr_sampler, coord, min(log2(steps * rough_factor + 1.0), 6.0)).rgb;
        }
        steps += 1.0;
    }
    return vec3(0.0, 0.0, 0.0);
}

void main() {
    vec3 position = texture(position_sampler, tex_coord).rgb;
    vec3 normal = normalize(texture(normal_sampler, tex_coord).rgb);
    vec3 pbr = texture(pbr_sampler, tex_coord).rgb;
    float metalness = texture(metalness_sampler, tex_coord).r;
    float roughness = texture(roughness_sampler, tex_coord).r;
    float depth = texture(depth_sampler, tex_coord).r;
    vec3 ray_pos = position;
    vec3 ray_dir = reflect(position - camera_pos, normal);
    float ray_step = 0.01;
    vec3 out_color = vec3(0.0, 0.0, 0.0);
    for(int i = 0; i < N_RAYS; i++) {
        out_color += ray_march(ray_pos, ray_dir, ray_step/* + (ray_step * rand(float(i + 1) * tex_pos.xy))*/, roughness);
    }
    out_color /= float(N_RAYS);
    color_output = vec4(out_color, 1.0);
}
