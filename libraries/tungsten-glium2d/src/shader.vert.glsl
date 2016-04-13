#version 140

uniform mat3 m_matrix;

in vec2 i_position;
in vec2 i_texture_coord;
in uint i_sampler_id;
in uint i_texture_id;

out vec2 v_texture_coord;
flat out uint v_sampler_id;
flat out uint v_texture_id;

void main() {
    v_texture_coord = i_texture_coord;
    v_sampler_id = i_sampler_id;
    v_texture_id = i_texture_id;
    gl_Position = vec4(m_matrix * vec3(i_position, 1.0), 1.0);
}
