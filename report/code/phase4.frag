vec3 final = hi;
vec3 temp = final;
for(int i = 0; i < MAX_ITERATIONS; i++) {
	vec3 mid = temp - dir * INITIAL_STEP_SIZE;
	vec4 ray_view_homo = camera * vec4(mid, 1.0);
	vec3 view = ray_view_homo.xyz / ray_view_homo.w;
	if(!(abs(view.x) <= 1.0 && abs(view.y) <= 1.0 && abs(view.z) <= 1.0)) {
		break;
	} else {
		vec2 coord = vec2(view.x + 1.0, view.y + 1.0) / 2.0;
		float depth = texture(depth_sampler, coord).r;
		if(view.z >= depth) {
			final = mid;
		}
		temp = mid;
	}
}
