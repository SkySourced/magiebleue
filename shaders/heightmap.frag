#version 330 core

out vec4 final_color;
in vec2 vTexCoords;
in vec3 vNormal;
in vec4 vColor;

uniform sampler2D heightmap;

void main() {
    float heightmap_val = texture(heightmap, vTexCoords).r;
    final_color = vColor;
}
