#version 410 core

out vec4 final_color;
in float height;

uniform sampler2D heightmap;

void main() {
    final_color = vec4(vec3(height), 1.0);
}
