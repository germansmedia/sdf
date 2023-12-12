// SDF - ray marching core
// by Desmond Germans, 2023

#include "formulas/mandelbox.glsl"
#include "formulas/menger3.glsl"
#include "formulas/amazingbox.glsl"
#include "formulas/amazingsurf.glsl"

#define ITERATE(formula) \
    if ((r >= uniforms.march.escape) || (iterations > uniforms.march.max_iterations)) { \
        return r / abs(dr); \
    } \
    formula(v,dr,p); \
    r = length(v); \
    iterations++;


// consult the fractal formulas
float consult(in vec3 p,inout uint iterations) {
    vec3 v = p;
    float dr = 1.0;
    float r = length(v);
    iterations = 0;
    ITERATE(mandelbox)
    ITERATE(amazingsurf)
    while ((r < uniforms.march.escape) && (iterations < uniforms.march.max_iterations)) {
        mandelbox(v,dr,p);
        r = length(v);
        iterations++;
    }
    return r / abs(dr);
}
