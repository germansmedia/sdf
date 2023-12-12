#ifndef _BASE_GLSL_
#define _BASE_GLSL_

#define PI 3.141592654
#define OOPI 0.318309886

struct Params {
    
    mat4 pose;  // camera pose inside fractal space

    vec4 forward_dir;  // measurement direction vector
    vec4 key_light_pos;  // key light position
    vec4 key_light_color;  // key light color
    vec4 shadow_power;  // multicolor shadow power

    vec4 sky_light_color;  // sky light color
    vec4 ambient_light_color;  // ambient light color
    vec4 background_color;  // background color
    vec4 glow_color;  // glow color
    
    vec4 palette[8];  // discrete palette

    float scale;  // The Scale
    float horizon;  // unscaled furthest distance
    float escape;  // iteration escape value
    float de_stop;  // closest distance to the fractal

    uint max_steps;  // maximum number of ray marching steps
    uint max_iterations;  // maximum number of iterations
    float iod;  // unscaled inter-occular distance
    uint tbd0;
};

#endif
