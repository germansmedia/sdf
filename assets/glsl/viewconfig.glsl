// SDF - view configuration
// by Desmond Germans, 2023

#ifndef _VIEWCONFIG_GLSL_
#define _VIEWCONFIG_GLSL_

// render quad with camera at fov
#define VIEW_TYPE_QUAD 0

// render cube map face
#define VIEW_TYPE_CUBE 1

// render cylinder map
#define VIEW_TYPE_CYLINDER 2

// render equirectangular map
#define VIEW_TYPE_EQUIRECT 3

// render fisheye map
#define VIEW_TYPE_FISHEYE 4

#define PI 3.1415927

struct ViewConfig {
    uint type_;  // one of VIEW_TYPE_*
    bool stereo;  // rendering stereo or not
    vec4 fov;  // FOV spec for VIEW_TYPE_QUAD
    ivec2 extent;  // output size in pixels
};

#endif
