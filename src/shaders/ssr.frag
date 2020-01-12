#version 300 es

precision mediump float;

#define MAX_ITERATIONS 64

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

vec4 cubic(float v) {
    vec4 n = vec4(1.0, 2.0, 3.0, 4.0) - v;
    vec4 s = n * n * n;
    float x = s.x;
    float y = s.y - 4.0 * s.x;
    float z = s.z - 4.0 * s.y + 6.0 * s.x;
    float w = 6.0 - x - y - z;
    return vec4(x, y, z, w) * (1.0/6.0);
}

vec4 textureBicubic(sampler2D sampler, vec2 texCoords, int level) {
   vec2 texSize = vec2(textureSize(sampler, level));
   vec2 invTexSize = 1.0 / texSize;

   texCoords = texCoords * texSize - 0.5;


    vec2 fxy = fract(texCoords);
    texCoords -= fxy;

    vec4 xcubic = cubic(fxy.x);
    vec4 ycubic = cubic(fxy.y);

    vec4 c = texCoords.xxyy + vec2 (-0.5, +1.5).xyxy;

    vec4 s = vec4(xcubic.xz + xcubic.yw, ycubic.xz + ycubic.yw);
    vec4 offset = c + vec4 (xcubic.yw, ycubic.yw) / s;

    offset *= invTexSize.xxyy;

    vec4 sample0 = textureLod(sampler, offset.xz, float(level));
    vec4 sample1 = textureLod(sampler, offset.yz, float(level));
    vec4 sample2 = textureLod(sampler, offset.xw, float(level));
    vec4 sample3 = textureLod(sampler, offset.yw, float(level));

    float sx = s.x / (s.x + s.y);
    float sy = s.z / (s.z + s.w);

    return mix(mix(sample3, sample2, sx), mix(sample1, sample0, sx), sy);
}

vec4 textureBicubicLod(sampler2D sampler, vec2 texCoords, float level) {
    float gap = fract(level);
    int base = int(level - gap);
    vec4 lower = textureBicubic(sampler, texCoords, base);
    vec4 upper = textureBicubic(sampler, texCoords, base + 1);
    return mix(lower, upper, gap);
}

vec3 ray_march(vec3 pos, vec3 dir, float rough_factor) {
    vec3 original_pos = pos;
    float steps = 0.0;
    const float STEP_FACTOR = 1e-3;
    float INITIAL_STEP_SIZE = max(abs(camera * vec4(dir * STEP_FACTOR, 0.0)), STEP_FACTOR);
    float step_size = INITIAL_STEP_SIZE;
    bool ok = false;
    for(int i = 0; i < MAX_ITERATIONS; i++) {
        pos += dir * step_size;
        vec4 ray_view_homo = camera * vec4(pos, 1.0);
        vec3 view = ray_view_homo.xyz / ray_view_homo.w;
        if(!(abs(view.x) <= 1.0 && abs(view.y) <= 1.0 && abs(view.z) <= 1.0)) {
            pos -= dir * step_size;
            step_size /= 2.0;
            continue;
        }
        vec2 coord = vec2(view.x + 1.0, view.y + 1.0) / 2.0;
        float depth = texture(depth_sampler, coord).r;
        if(view.z >= depth) {
            ok = true;
            break;
        }
        step_size *= 2.0;
    }
    if(!ok) {
        return vec3(0.0, 0.0, 0.0);
    }
    vec3 hi = pos;
    vec3 lo = pos - dir * step_size;
    for(int i = 0; i < MAX_ITERATIONS; i++) {
        vec3 mid = (lo + hi) / 2.0;
        vec4 ray_view_homo = camera * vec4(mid, 1.0);
        vec3 view = ray_view_homo.xyz / ray_view_homo.w;
        if(!(abs(view.x) <= 1.0 && abs(view.y) <= 1.0 && abs(view.z) <= 1.0)) {
            hi = mid;
        } else {
            vec2 coord = vec2(view.x + 1.0, view.y + 1.0) / 2.0;
            float depth = texture(depth_sampler, coord).r;
            if(view.z >= depth) {
                hi = mid;
            } else {
                lo = mid;
            }
        }
    }
    vec3 final = hi;
    vec4 ray_view_homo = camera * vec4(final, 1.0);
    vec3 view = ray_view_homo.xyz / ray_view_homo.w;
    if(!(abs(view.x) <= 1.0 && abs(view.y) <= 1.0 && abs(view.z) <= 1.0)) {
        return vec3(0.0, 0.0, 0.0);
    }
    vec2 coord = vec2(view.x + 1.0, view.y + 1.0) / 2.0;
    float depth = texture(depth_sampler, coord).r;
    if(view.z >= depth) {
        float dist = abs(original_pos - final);
        return textureBicubicLod(pbr_sampler, coord, clamp(/*pow(dist, 1.25)*/0.0, 0.0, 8.0)).rgb;
    } else {
        return vec3(0.0, 0.0, 0.0);
    }
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
    vec3 out_color = ray_march(ray_pos, ray_dir, roughness);
    color_output = vec4(out_color, 1.0);
}
