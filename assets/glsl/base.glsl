#ifndef _BASE_GLSL_
#define _BASE_GLSL_

#define PI 3.1415927

#define ANISOTROPY_SQUARE 0
#define ANISOTROPY_RECT2 1
#define ANISOTROPY_RECT4 2
#define ANISOTROPY_RECT8 3
#define ANISOTROPY_RECT16 4

#define PHASE_FULL16X16 0
#define PHASE_RIGHT8X16 1
#define PHASE_BOTTOM8X8 2
#define PHASE_RIGHT4X8  3
#define PHASE_BOTTOM4X4 4
#define PHASE_RIGHT2X4  5
#define PHASE_BOTTOM2X2 6
#define PHASE_RIGHT1X2  7
#define PHASE_BOTTOM1X1 8

#define VIEW_TYPE_QUAD 0
#define VIEW_TYPE_STEREO_QUAD 1
#define VIEW_TYPE_CUBE 2
#define VIEW_TYPE_STEREO_CUBE 3
#define VIEW_TYPE_CYLIDER 4
#define VIEW_TYPE_STEREO_CYLINDER 5
#define VIEW_TYPE_EQUIRECT 6
#define VIEW_TYPE_STEREO_EQUIRECT 7
#define VIEW_TYPE_FISHEYE 8
#define VIEW_TYPE_STEREO_FISHEYE 9

struct ViewConfig {
    uint width;
    uint height;
    uint type_;
    uint tbd0;

    vec4 fov;
};

struct Progress {
    uint phase;  // one of PHASE_*
    uint x,y;  // pixel offset of this block/pixel
    uint tbd0;
};

struct March {

    mat4 pose;  // pose inside fractal space

    float scale;  // scale factor
    float horizon;  // furthest horizon
    float escape;  // escape value
    float de_stop;  // MB3D de_stop

    float de_stop_factor;  // MB3D de_stop_factor
    uint max_steps;  // maximum number of marching steps
    uint max_iterations;  // maximum number of iterations
    uint tbd0;

    vec4 forward_dir;  // view direction (based on head orientation)
};

struct Render {
    vec4 albedo_color;
    vec4 key_light_pos;
    vec4 key_light_color;
    vec4 shadow_power;
    vec4 sky_light_color;
    vec4 ambient_light_color;
    vec4 background_color;
    vec4 glow_color;
};

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

#endif
