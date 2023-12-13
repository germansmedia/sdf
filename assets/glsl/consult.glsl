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
    formula(v,dr,p); \
    r = length(v); \
    iterations++;

#define INFINITERATE(formula) \
    while ((r < uniforms.params.escape) && (iterations < uniforms.params.max_iterations)) { \
        formula(v,dr,p); \
        r = length(v); \
        iterations++; \
    }

// consult the fractal formulas
float consult(in vec3 p,inout uint iterations) {
    vec3 v = p;
    float dr = 1.0;
    float r = length(v);
    iterations = 0;
    INFINITERATE(menger3)
    return r / abs(dr);
}
