#version 330

in vec2 fragTexCoord;
in vec4 fragColor;

out vec4 finalColor;

uniform sampler2D texture0;
uniform vec4 colDiffuse;
uniform float time;
uniform float noise_scale;
uniform float intensity;
uniform float alpha;
uniform vec3 color_hot;
uniform vec3 color_mid;
uniform vec3 color_cool;

float hash(vec2 p) {
    return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453123);
}

float noise(vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    float a = hash(i);
    float b = hash(i + vec2(1.0, 0.0));
    float c = hash(i + vec2(0.0, 1.0));
    float d = hash(i + vec2(1.0, 1.0));
    vec2 u = f * f * (3.0 - 2.0 * f);
    return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}

float fbm(vec2 p) {
    float value = 0.0;
    float amplitude = 0.6;
    float frequency = 1.0;
    for (int i = 0; i < 3; i++) {
        value += noise(p * frequency) * amplitude;
        frequency *= 2.0;
        amplitude *= 0.5;
    }
    return value;
}

void main() {
    vec2 uv = fragTexCoord;
    float t = time * 1.6;

    vec2 swirl = vec2(
        sin((uv.y + t * 0.35) * 6.0),
        cos((uv.x + t * 0.25) * 4.0)
    );
    vec2 flow = (uv + swirl * 0.08) * vec2(noise_scale, noise_scale * 1.6);

    float n = fbm(flow + vec2(t * 0.7, -t * 0.35));
    float streaks = fbm(flow * 1.6 + vec2(-t * 0.5, t * 0.9));
    float flicker = clamp(mix(n, streaks, 0.55), 0.0, 1.0);

    float edge_dist = abs(uv.y - 0.5) * 2.0;
    float core = 1.0 - smoothstep(0.2, 1.0, edge_dist);
    float flame = clamp(core + (flicker - 0.45) * 1.4, 0.0, 1.0);

    float front_fade = smoothstep(0.0, 0.06, uv.x);
    float back_fade = 1.0 - smoothstep(0.78, 1.0, uv.x);
    float span_fade = front_fade * back_fade;

    float hot_mask = smoothstep(0.45, 1.0, flame + streaks * 0.3);
    float mid_mask = smoothstep(0.15, 0.85, flame + flicker * 0.2);

    vec3 color = mix(color_cool, color_mid, mid_mask);
    color = mix(color, color_hot, hot_mask);

    float out_alpha = alpha * flame * span_fade;
    vec4 base = texture(texture0, fragTexCoord) * colDiffuse;
    finalColor = vec4(color * intensity, out_alpha) * base * fragColor;
}
