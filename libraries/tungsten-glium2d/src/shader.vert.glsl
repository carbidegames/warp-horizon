#version 140

uniform mat3 m_matrix;

in vec2 i_position;

void main() {
    gl_Position = vec4(m_matrix * vec3(i_position, 1.0), 1.0);
}
