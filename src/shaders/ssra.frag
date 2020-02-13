#version 300 es

precision mediump float;

out vec4 color_output;

in vec2 tex_coord;

uniform sampler2D ssr_sampler;
uniform sampler2D metalness_sampler;
uniform sampler2D pbr_sampler;
uniform sampler2D roughness_sampler;

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

void main() {
    float metalness = texture(metalness_sampler, tex_coord).r;
    float roughness = texture(roughness_sampler, tex_coord).r;
    vec3 base_color = textureLod(pbr_sampler, tex_coord, 0.0).rgb;
    vec3 ssr = textureBicubicLod(ssr_sampler, tex_coord, roughness * 4.0).rgb;
    if(ssr.b >= 1e19) {
        color_output = vec4(base_color, 1.0);
    } else {
        float dl = ssr.b / 1e1;
        vec3 reflection = textureBicubicLod(pbr_sampler, ssr.rg, roughness * dl * 4.0).rgb;
        color_output = vec4(reflection, 1.0);
    }
}
