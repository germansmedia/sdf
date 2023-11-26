// SDF - lighting stage compute shader
// by Desmond Germans, 2023

#version 450

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

#include "progress.glsl"
#include "march.glsl"
#include "render.glsl"

// 0 = uniforms, see base.glsl

layout (binding = 1,rg32f) readonly uniform image2D do_image;

layout (binding = 2) writeonly uniform image2D rgba_image;

void main() {
    // TODO: run ray march progress, writing to image (RGBA)
}
