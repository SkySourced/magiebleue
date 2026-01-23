#version 410 core

layout(vertices = 4) out;

// from vertex shader 
in vec2 vTexCoord[];
in vec3 vNormal[];

// to tese shader
out vec2 fTexCoord[];
out vec3 fNormal[];

void main() {
    // Pass through values for each vertex call
    gl_out[gl_InvocationID].gl_Position = gl_in[gl_InvocationID].gl_Position;
    fTexCoord[gl_InvocationID] = vTexCoord[gl_InvocationID];
    fNormal[gl_InvocationID] = vNormal[gl_InvocationID];
    
    if (gl_InvocationID == 0) {
        // set tessellation levels only on the first vertex
        gl_TessLevelOuter[0] = 16;
        gl_TessLevelOuter[1] = 16;
        gl_TessLevelOuter[2] = 16;
        gl_TessLevelOuter[3] = 16;
        
        gl_TessLevelInner[0] = 16;
        gl_TessLevelInner[1] = 16;
    }
}
