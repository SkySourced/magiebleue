#version 330 core

out vec4 final_color;
in vec2 v_texCoords;
uniform sampler2D heightmap;

void main() {
    float heightmap_val = texture(heightmap, v_texCoords).r;
    final_color = vec4(vec3(heightmap_val), 1.0);
}
