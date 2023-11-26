// SDF - lighting core
// by Desmond Germans, 2023

#ifndef _RENDER_GLSL_
#define _RENDER_GLSL_

#include "march.glsl"

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

vec4 process_lighting(in March march,in Render render,vec3 p,vec3 dp) {
    // TODO!!
    return vec4(0.0,0.0,0.0,1.0);
}

#endif
