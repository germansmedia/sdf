// SDF - lighting stage compute shader
// by Desmond Germans, 2023

#version 450

#include "viewconfig.glsl"
#include "progress.glsl"
#include "march.glsl"
#include "render.glsl"

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

layout (std140,push_constant) readonly uniform Push {
    uint eye;
    uint face;
    uint anisotropy;
    uint y_offset;
} push;

layout (std140,binding = 0) readonly uniform Uniforms {
    ViewConfig view;
    Progress progress;
    // formula params
    March march;
    Render render;
} uniforms;

layout (binding = 1,rg32f) readonly uniform image2D do_image;

layout (binding = 2) writeonly uniform image2D rgba_image;

void main() {
    // TODO: run ray march progress, writing to image (RGBA)
}
