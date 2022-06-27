#version 330

in vec3 position;

uniform mat4 translation;
uniform mat4 view;

void main() {
    gl_Position = view * translation * vec4(position, 1.0);
}
