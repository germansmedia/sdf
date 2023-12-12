#ifndef _BASE_GLSL_
#define _BASE_GLSL_

#define PI 3.141592654
#define OOPI 0.318309886

struct March {

    mat4 pose;  // pose inside fractal space

    float scale;  // scale factor
    float horizon;  // unscaled furthest horizon
    float escape;  // escape value
    float de_stop;  // unscaled MB3D de_stop

    float de_stop_factor;  // unscaled MB3D de_stop_factor
    uint max_steps;  // maximum number of marching steps
    uint max_iterations;  // maximum number of iterations
    float iod;  // unscaled distance between left and right eyes

    vec4 forward_dir;  // view direction (based on head orientation, used for distance measurement only)
};

struct Render {
    vec4 key_light_pos;
    vec4 key_light_color;
    vec4 shadow_power;
    vec4 sky_light_color;

    vec4 ambient_light_color;
    vec4 background_color;
    vec4 glow_color;
    vec4 tbd0;

    vec4 palette[4];
};

#endif
