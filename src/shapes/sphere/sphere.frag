#version 330

uniform vec3 color;
uniform sampler2D tex;

in vec2 v_tex_coords;
out vec4 fragColor;

void main() {

    fragColor = vec4(1.0, 1.0, 1.0, 1.0);
}
