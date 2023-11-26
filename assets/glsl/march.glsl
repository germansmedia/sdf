// SDF - ray marching core
// by Desmond Germans, 2023

#ifndef _MARCH_GLSL_
#define _MARCH_GLSL_

#include "formulas/mandelbox.glsl"

// March parameters
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
};

// consult the fractal formulas
float consult(in March march,in vec3 p,inout uint iterations) {
    vec3 v = p;
    float dr = 1.0;
    float r = length(v);
    uint i = 0;
    for (; (r < march.escape) && (i <= march.max_iterations); i++) {
        mandelbox(v,dr,p);
        r = length(v);
    }
    return r / abs(dr);
}

// march a single ray
bool march_ray(
    in March march,
    inout vec3 p,  // ray start and end
    in vec3 dp,  // march direction
    out float r,  // distance to object
    out uint steps  // how many steps were taken
) {
    float closest = march.horizon;
    steps = 1;
    float de_stop = march.de_stop;
    bool hit = false;
    uint iterations = 0;
    float de = consult(march,p,iterations);
    if ((iterations > march.max_iterations) || (de < march.de_stop)) {
        r = de;
        return true;
    }
    else {
        float variation = de;
        while (r < march.horizon) {
            if (iterations > march.max_iterations) {
                float h = -0.5 * variation;
                r += h;
                p += h * dp;
                de_stop = march.de_stop * (1.0 + march.de_stop_factor * r);
                de = consult(march,p,iterations);
                variation = -h;
            }
            if ((iterations > march.max_iterations) || (de < de_stop)) {
                r = 0.0;
                return true;
            }
            float last_de = de;
            r += de;
            p += de * dp;
            de_stop = march.de_stop * (1.0 + march.de_stop_factor * r);
            de = consult(march,p,iterations);
            if (de > last_de + variation) {
                de = last_de + variation;
            }
            closest = min(closest,de);
            steps += 1;
        }
    }
    r = closest;
    return false;
}

// measure depth
float measure_depth(in March march,in vec3 p,in vec3 dp) {
    float r;
    uint steps;
    if (march_ray(march,p,dp,r,steps)) {
        return r;
    }
    else {
        return march.horizon;
    }
}

// process depth/occlusion ray
vec2 process_depth_occlusion(in March march,vec3 p,vec3 dp) {
    float r;
    uint steps;
    if (march_ray(march,p,dp,r,steps)) {
        float occlusion = 1.0 - clamp(float(steps) / float(march.max_steps),0.0,1.0);
        return vec2(r,occlusion);
    }
    else {
        return vec2(march.horizon,1.0);
    }
}

#endif
