#version 140

uniform sampler2DArray m_textures;

in vec2 v_tex;
in float v_texid;

out vec4 o_color;

void main() {
    o_color = texture(m_textures, vec3(v_tex, v_texid));
}
