#version 410 core

layout(quads, fractional_odd_spacing, ccw) in;

uniform sampler2D heightmap;
uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

in vec2 fTexCoord[];
in vec3 fNormal[];

out float height;

void main() {
    // offset from patch origin
    vec2 patch_coord = gl_TessCoord.xy;

    vec2 t00 = fTexCoord[0];
    vec2 t01 = fTexCoord[1];
    vec2 t10 = fTexCoord[2];
    vec2 t11 = fTexCoord[3];

    // bilinearly interpolate to find tessellated texcoord
    vec2 t0 = (t01 - t00) * patch_coord.x + t00;
    vec2 t1 = (t11 - t10) * patch_coord.x + t10;
    vec2 texCoord = (t1 - t0) * patch_coord.y + t0;

    // retrieve value from heightmap
    height = texture(heightmap, texCoord).r;

    // bilerp vertex position
    vec4 p00 = gl_in[0].gl_Position;
    vec4 p01 = gl_in[1].gl_Position;
    vec4 p10 = gl_in[2].gl_Position;
    vec4 p11 = gl_in[3].gl_Position;

    vec4 p0 = (p01 - p00) * patch_coord.x + p00;
    vec4 p1 = (p11 - p10) * patch_coord.x + p10;
    vec4 pos = (p1 - p0) * patch_coord.y + p0;

    // bilerp vertex normal
    vec3 n00 = fNormal[0];
    vec3 n01 = fNormal[1];
    vec3 n10 = fNormal[2];
    vec3 n11 = fNormal[3];

    vec3 n0 = (n01 - n00) * patch_coord.x + n00;
    vec3 n1 = (n11 - n10) * patch_coord.x + n10;
    vec3 normal = (n1 - n0) * patch_coord.y + n0;
    
    // apply heightmap along normal
    pos += vec4(normal, 0.0) * height * 24.0;
    
    // apply MVP to project to clip space
    gl_Position = proj * view * model * pos;
}
