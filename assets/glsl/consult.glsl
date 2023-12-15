// SDF - ray marching core
// by Desmond Germans, 2023

#include "formulas/mandelbox.glsl"
#include "formulas/menger3.glsl"
#include "formulas/amazingbox.glsl"
#include "formulas/amazingsurf.glsl"

#define ITERATE(formula) \
    if ((r >= uniforms.params.escape) || (iterations > uniforms.params.max_iterations)) { \
        return r / abs(dr); \
    } \
    formula(v,c,dr); \
    r = length(v); \
    iterations++;

#define INFINITERATE(formula) \
    while ((r < uniforms.params.escape) && (iterations < uniforms.params.max_iterations)) { \
        formula(v,c,dr); \
        r = length(v); \
        iterations++; \
    }

// consult the fractal formulas
float consult(in vec3 p,inout uint iterations) {
    vec3 v = p;
    vec3 c = p;
    float dr = 1.0;
    float r = length(v);
    iterations = 0;
    ITERATE(amazingsurf)
    ITERATE(mandelbox)
    INFINITERATE(amazingbox)
    return r / abs(dr);
}

// consult the fractal formulas (Julia)
float consult_julia(in vec3 p,in vec3 init,inout uint iterations) {
    vec3 v = init;
    vec3 c = p;
    float dr = 1.0;
    float r = length(c);
    iterations = 0;
    INFINITERATE(mandelbox)
    return r / abs(dr);
}
