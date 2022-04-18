#version 330

uniform vec3 color;

in vec2 v_tex_coords;
out vec4 fragColor;

uniform sampler2D tex;

void main() {

    fragColor = texture(tex, v_tex_coords);
    //fragColor = vec4(color, 1.0);
}
