#version 140

out vec4 color;

void main() {
    float lerpValue = gl_FragCoord.y / 720.0f;
	color = mix(vec4(1.0f, 0.0f, 0.0f, 1.0f), vec4(0.0f, 1.0f, 0.0f, 1.0f), lerpValue);
    // above is just a quick test with gammas and such
    //color = vec4(1.0, 0.0, 0.0, 1.0);
}
