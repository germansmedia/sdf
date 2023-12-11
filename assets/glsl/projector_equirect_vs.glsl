// SDF - equirectangular draw vertex shader
// by Desmond Germans, 2023

#version 450

layout (location = 0) in vec2 vertex;

layout (location = 0) out vec3 direction;

layout (std140,binding = 0) uniform Uniforms {
    mat4 matrix;
    vec4 fovs[2];
} uniforms;

layout (std140,push_constant) uniform Push {
    uint eye;
} push;

void main() {

    // get FOV tan
    vec4 tan_fov = tan(uniforms.fovs[push.eye]);
    float x = 0.5 * (-tan_fov.x + tan_fov.y) * vertex.x + 0.5 * (-tan_fov.x - tan_fov.y);
    float y = 0.5 * (-tan_fov.z + tan_fov.w) * vertex.y + 0.5 * (-tan_fov.z - tan_fov.w);

    // construct direction vector
    direction = normalize((uniforms.matrix * vec4(x,y,-1.0,1.0)).xyz);

    // set rasterization position
    gl_Position = vec4(vertex,0.0,1.0);
}
