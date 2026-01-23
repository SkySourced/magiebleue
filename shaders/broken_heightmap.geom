#version 330 core
layout(lines) in;
layout(triangle_strip, max_vertices = 16) out;

in VS_OUT {
    vec2 texCoords;
    vec3 normal;
} gs_in[];

out vec2 vTexCoords;
out vec3 vNormal;
out vec4 vColor;

uniform mat4 view;
uniform mat4 proj;
uniform sampler2D heightmap;

mat2 rotation2d(float angle) {
    float s = sin(angle);
    float c = cos(angle);

    return mat2(
        c, -s,
        s, c
    );
}

mat4 rotation3d(vec3 axis, float angle) {
    axis = normalize(axis);
    float s = sin(angle);
    float c = cos(angle);
    float oc = 1.0 - c;

    return mat4(
        oc * axis.x * axis.x + c, oc * axis.x * axis.y - axis.z * s, oc * axis.z * axis.x + axis.y * s, 0.0,
        oc * axis.x * axis.y + axis.z * s, oc * axis.y * axis.y + c, oc * axis.y * axis.z - axis.x * s, 0.0,
        oc * axis.z * axis.x - axis.y * s, oc * axis.y * axis.z + axis.x * s, oc * axis.z * axis.z + c, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
}

vec4 pos_base;
vec4 pos_i_unit;
vec4 pos_j_unit;
vec2 texcoord_base;
vec2 texcoord_i_unit;
vec2 texcoord_j_unit;
vec3 normal_base;
vec3 normal_i_unit;
vec3 normal_j_unit;

void buildVertex(int i, int j) {
    vec2 texcoord = texcoord_base + i * texcoord_i_unit + j * texcoord_j_unit;
    vTexCoords = texcoord;
    vColor = vec4(i / 2.0, j / 2.0, 0.0, 1.0);
    vec3 normal = normal_base + i * normal_i_unit + j * normal_j_unit;
    vNormal = normal;
    vec4 position = (pos_base + i * pos_i_unit + j * pos_j_unit) /* + normal * texture(heightmap, texcoord).r*/ ;
    gl_Position = position;
}

void main() {
    vec3 up = (proj * view * vec4(0.0, 1.0, 0.0, 0.0)).xyz;
    pos_base = gl_in[0].gl_Position;
    pos_i_unit = (gl_in[0].gl_Position - gl_in[1].gl_Position) / 2.0;
    pos_j_unit = pos_i_unit * rotation3d(up, radians(90.0));

    texcoord_base = gs_in[0].texCoords;
    texcoord_i_unit = (gs_in[0].texCoords - gs_in[1].texCoords) / 2.0;
    texcoord_j_unit = texcoord_i_unit * rotation2d(radians(90.0));

    normal_base = gs_in[0].normal;
    normal_i_unit = (gs_in[0].normal - gs_in[1].normal) / 2.0;
    normal_j_unit = (vec4(normal_i_unit, 0.0) * rotation3d(up, radians(90.0))).xyz;

    for (int i = 0; i < 2; i++) { // x (original unit)
        for (int j = 0; j < 2; j++) { // y (rotated 90deg around global y axis)
            buildVertex(i, j);
            EmitVertex();

            buildVertex(i + 1, j);
            EmitVertex();

            buildVertex(i, j + 1);
            EmitVertex();
            EndPrimitive();
        }
    }
}
