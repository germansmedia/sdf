#ifndef _TILER_GLSL_
#define _TILER_GLSL_

#include "base.glsl"

#define TYPE_QUAD 0
#define TYPE_CUBE 1
#define TYPE_CYLIDER 2
#define TYPE_EQUIRECT 3
#define TYPE_FISHEYE 4

#define FLAGS_STEREO 0x0001

struct Config {

    uint type_;
    uint flags;
    uint tile_width,tile_height;

    uint tile_count_x,tile_count_y;
    uint current_tile_x,current_tile_y;

    vec4 fovs[2];
};

layout (std140,push_constant) readonly uniform Push {
    uint eye;
    uint face;
} push;

layout (std140,binding = 0) readonly uniform Uniforms {
    Config config;
    // formula params
    March march;
    Render render;
} uniforms;

#endif
