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
