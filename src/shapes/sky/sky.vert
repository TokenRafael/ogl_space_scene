#version 330

in vec3 position;

uniform mat4 translation;

void main() {
    gl_Position = translation * vec4(position, 1.0);
}
