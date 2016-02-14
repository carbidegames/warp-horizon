#version 140

// uniform mat4 u_model; Unused
uniform mat4 u_view;
uniform mat4 u_projection;

in vec3 position;
in vec3 normal;
in vec3 color;

out vec3 v_color;
out vec3 v_normal;

void main() {
    v_color = color;
    mat4 modelview = u_view/* * u_model Perhaps not calculate this in shader?*/;
    v_normal = transpose(inverse(mat3(modelview))) * normal;
    gl_Position = u_projection * modelview * vec4(position, 1.0);
}
