// SDF - distance measurement compute shader
// by Desmond Germans, 2023

# version 450

#include "viewconfig.glsl"
#include "march.glsl"

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

layout (std140,push_constant) readonly uniform Push {
    uint eye;
    uint face;
    uint anisotropy;
    uint y_offset;
} push;

layout (std140,binding = 0) readonly uniform Uniforms {
    ViewConfig view;
    // formula params
    March march;
} uniforms;

layout (binding = 1) writeonly buffer Buffer {
    float depth;
} storage;

void main() {

    // get direction
    vec3 origin = (uniforms.march.pose * vec4(0.0,0.0,0.0,1.0)).xyz;
    vec3 view = (uniforms.march.pose * vec4(0.0,0.0,-1.0,1.0)).xyz;
    vec3 dir = normalize(view - origin);

    // measure distance
    storage.depth = measure_depth(uniforms.march,origin,dir);
}
