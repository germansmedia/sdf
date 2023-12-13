// SDF - distance measurement compute shader
// by Desmond Germans, 2023

#version 450

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

// base March and Render structs
#include "base.glsl"

// uniforms and push constants
struct Config {
    uint width,height;
    uint tbd0;
    uint tbd1;
};

layout (std140,binding = 0) readonly uniform Uniforms {
    Config config;
    Params params;
} uniforms;

// ray marching code
#include "march.glsl"

// output storage buffer
layout (binding = 1) writeonly buffer Buffer {
    float depth;
} storage;

void main() {

    // get directions
    vec3 origin = (uniforms.params.pose * vec4(0.0,0.0,0.0,1.0)).xyz;
    vec3 forward = (uniforms.params.pose * vec4(uniforms.params.forward_dir.xyz,1.0)).xyz;
    vec3 dir = normalize(forward - origin);

    // calculate solid angle per pixel
    // TODO: also take FOV into account here
    float sr_per_pixel = 1.0 / (float(uniforms.config.width) * float(uniforms.config.height));

    // measure distances
    storage.depth = measure_depth(origin,dir,sr_per_pixel);
}
