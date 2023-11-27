// SDF - ray marching core
// by Desmond Germans, 2023

#include "base.glsl"

#include "formulas/mandelbox.glsl"
#include "formulas/menger3.glsl"

// consult the fractal formulas
float consult(in vec3 p,inout uint iterations) {
    vec3 v = p;
    float dr = 1.0;
    float r = length(v);
    uint i = 0;
    for (; (r < uniforms.march.escape) && (i <= uniforms.march.max_iterations); i++) {
        menger3(v,dr,p);
        r = length(v);
    }
    return r / abs(dr);
}

// march a single ray
bool march_ray(
    in vec3 p,  // ray start
    in vec3 dp,  // march direction
    out float r,  // distance to object
    out uint steps  // how many steps were taken
) {
    float closest = uniforms.march.horizon;
    steps = 0;
    float de_stop = uniforms.march.de_stop;
    bool hit = false;
    uint iterations = 0;
    float de = consult(p,iterations);
    r = de;
    if ((iterations > uniforms.march.max_iterations) || (de < de_stop)) {
        return true;
    }
    else {
        float variation = de;
        while (r < uniforms.march.horizon) {
            if (iterations > uniforms.march.max_iterations) {
                float h = -0.5 * variation;
                r += h;
                p += h * dp;
                de_stop = uniforms.march.de_stop * (1.0 + uniforms.march.de_stop_factor * r);
                de = consult(p,iterations);
                variation = -h;
            }
            r += de;
            if ((iterations > uniforms.march.max_iterations) || (de < de_stop)) {
                return true;
            }
            float last_de = de;
            p += de * dp;
            de_stop = uniforms.march.de_stop * (1.0 + uniforms.march.de_stop_factor * r);
            de = consult(p,iterations);
            if (de > last_de + variation) {
                de = last_de + variation;
            }
            closest = min(closest,de);
            steps += 1;
        }
    }
    r = de;
    return false;
}

// measure depth
float measure_depth(in vec3 p,in vec3 dp) {
    float r;
    uint steps;
    if (march_ray(p,dp,r,steps)) {
        return r;
    }
    else {
        return uniforms.march.horizon;
    }
}

// process depth/occlusion ray
vec2 process_depth_occlusion(vec3 p,vec3 dp) {
    float r;
    uint steps;
    if (march_ray(p,dp,r,steps)) {
        float occlusion = 1.0 - clamp(float(steps) / float(uniforms.march.max_steps),0.0,1.0);
        return vec2(r,occlusion);
    }
    else {
        return vec2(uniforms.march.horizon,-1.0);
    }
}
