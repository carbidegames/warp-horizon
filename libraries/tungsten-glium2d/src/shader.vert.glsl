#version 140

uniform mat3 m_matrix;

in vec2 i_position;
in vec2 i_tex;
in float i_texid;

out vec2 v_tex;
out float v_texid;

void main() {
    v_tex = i_tex;
    v_texid = i_texid;
    gl_Position = vec4(m_matrix * vec3(i_position, 1.0), 1.0);
}
