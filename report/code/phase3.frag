vec3 hi = nextpos;
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
