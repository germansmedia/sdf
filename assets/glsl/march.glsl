// SDF - ray marching core
// by Desmond Germans, 2023

#include "base.glsl"

#include "formulas/mandelbox.glsl"
#include "formulas/menger3.glsl"
#include "formulas/amazingbox.glsl"

// Agressive Stance color scheme
#define AS_ONE vec3(0.0275,0.2196,0.2588)
#define AS_TWO vec3(0.0431,0.3647,0.3176)
#define AS_THREE vec3(0.9098,0.6392,0.4157)
#define AS_FOUR vec3(0.8314,0.2627,0.3490)
#define AS_FIVE vec3(0.6118,0.1922,0.3804)

vec3 color_scheme(float f) {
    uint i = uint(floor(4.0 * clamp(f,0.0,1.0)));
    float r = fract(4.0 * f);
    switch(i) {
        case 0: return mix(AS_ONE,AS_TWO,r);
        case 1: return mix(AS_TWO,AS_THREE,r);
        case 2: return mix(AS_THREE,AS_FOUR,r);
        case 3: return mix(AS_FOUR,AS_FIVE,r);
        case 4: return AS_FIVE;
    }
}

// consult the fractal formulas
float consult(in vec3 p,inout uint iterations) {
    vec3 v = p;
    float dr = 1.0;
    float r = length(v);
    iterations = 0;
    for (; (r < uniforms.march.escape) && (iterations <= uniforms.march.max_iterations); iterations++) {
        amazingbox(v,dr,p);
        r = length(v);
    }
    return r / abs(dr);
}

// march a single ray
bool march_ray(
    in vec3 p,  // ray start
    in vec3 dp,  // march direction
    out float r,  // distance to object
    out uint steps,  // how many steps were taken
    out uint iterations  // how many iterations were needed
) {
    float closest = uniforms.march.horizon;
    steps = 0;
    r = 0.0;
    float de_stop = uniforms.march.de_stop;
    bool hit = false;
    iterations = 0;
    float de = consult(p,iterations);
    if ((iterations > uniforms.march.max_iterations) || (de < de_stop)) {
        r = de;
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
            if ((iterations > uniforms.march.max_iterations) || (de < de_stop)) {
                return true;
            }
            float last_de = de;
            r += de;
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
    r = closest;
    return false;
}

// measure depth
float measure_depth(in vec3 p,in vec3 dp) {
    float r;
    uint steps;
    uint iterations;
    if(march_ray(p,dp,r,steps,iterations)) {
        return r;
    }
    else {
        return uniforms.march.horizon;
    }
}

// process depth/occlusion ray
vec4 process_dosi(vec3 p,vec3 dp) {
    float r;
    uint steps;
    uint iterations;
    if (march_ray(p,dp,r,steps,iterations)) {
        float occlusion = 1.0 - clamp(float(steps) / float(uniforms.march.max_steps),0.0,1.0);
        return vec4(r,occlusion,float(steps),float(iterations));
    }
    else {
        return vec4(uniforms.march.horizon,-1.0,0.0,0.0);
    }
}
