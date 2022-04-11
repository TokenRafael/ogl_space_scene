#version 330

in vec3 position;

uniform mat4 translation;
uniform mat4 rotation;
uniform mat4 scale;
uniform mat4 self_rotation;

void main() {
    gl_Position =
        self_rotation *
        translation *
        rotation *
        scale *
        vec4(position, 1.0);
}
