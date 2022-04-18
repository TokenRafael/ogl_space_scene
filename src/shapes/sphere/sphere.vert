#version 330

in vec3 position;
in vec2 tex_coords;
out vec2 v_tex_coords;

uniform mat4 translation;
uniform mat4 rotation;
uniform mat4 scale;
uniform mat4 self_rotation;

void main() {
    v_tex_coords = tex_coords;
    gl_Position =
        self_rotation *
        translation *
        rotation *
        scale *
        vec4(position, 1.0);
}
