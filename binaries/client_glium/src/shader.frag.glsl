#version 140

uniform vec3 u_light_dir;

in vec3 v_color;
in vec3 v_normal;

out vec4 o_color;

void main() {
    float brightness = dot(normalize(v_normal), normalize(u_light_dir));
    vec3 dark_color = v_color * 0.6;
    o_color = vec4(mix(dark_color, v_color, brightness), 1.0);
}
