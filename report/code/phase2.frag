vec3 nextpos = pos;
for(int i = MAX_ITERATIONS; i > 0; i--) {
	float alpha = float(i) / float(MAX_ITERATIONS);
	vec3 testpos = original_pos * (1.0 - alpha) + pos * alpha;
	vec4 ray_view_homo = camera * vec4(testpos, 1.0);
	vec3 view = ray_view_homo.xyz / ray_view_homo.w;
	if(abs(view.x) <= 1.0 && abs(view.y) <= 1.0 && abs(view.z) <= 1.0) {
		vec2 coord = vec2(view.x + 1.0, view.y + 1.0) / 2.0;
		float depth = texture(depth_sampler, coord).r;
		if(view.z >= depth) {
			nextpos = testpos;
		}
	}
}
