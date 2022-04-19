#version 330

in vec3 position;
in vec2 tex_coords;
out vec2 v_tex_coords;

uniform mat4 translation;
uniform mat4 undo_translation;
uniform mat4 rotation;
uniform mat4 scale;
uniform mat4 self_rotation;

void main() {
    v_tex_coords = tex_coords;
    vec4 rotated = self_rotation * vec4(position, 1.0);
    gl_Position =
    translation *
    rotation *
    scale *
    rotated;
}
