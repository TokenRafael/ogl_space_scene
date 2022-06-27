#version 330

in vec3 position;

uniform mat4 translation;
uniform mat4 undo_translation;
uniform mat4 rotation;
uniform mat4 scale;
uniform mat4 self_rotation;
uniform mat4 view;

void main() {
    // Operations occur from right to left
    gl_Position =
    view *
    rotation *
    translation *
    scale *
    self_rotation *
    vec4(position, 1.0);
}
