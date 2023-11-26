// SDF - distance measurement compute shader
// by Desmond Germans, 2023

#version 450

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

#include "march.glsl"

// 0 = uniforms, see base.glsl

layout (binding = 1) writeonly buffer Buffer {
    float depth;
} storage;

void main() {

    // get direction
    vec3 origin = (uniforms.march.pose * vec4(0.0,0.0,0.0,1.0)).xyz;
    vec3 view = (uniforms.march.pose * vec4(0.0,0.0,-1.0,1.0)).xyz;
    vec3 dir = normalize(view - origin);

    // measure distance
    storage.depth = measure_depth(origin,dir);
}
