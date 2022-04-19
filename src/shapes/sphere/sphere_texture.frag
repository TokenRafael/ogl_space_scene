#version 330

uniform vec3 color;
uniform sampler2D tex;

in vec2 v_tex_coords;
out vec4 frag_texture;

void main() {

    frag_texture = texture(tex, v_tex_coords);
}
