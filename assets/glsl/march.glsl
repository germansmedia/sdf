// SDF - ray marching core
// by Desmond Germans, 2023

// consult the fractal
#include "consult.glsl"

/*
// march a single ray
bool march_ray(
    in vec3 p,  // ray start
    in vec3 dp,  // march direction
    out float r,  // distance to object
    out uint steps,  // how many steps were taken
    out uint iterations  // how many iterations were needed
) {
    // initialize values
    r = 0.0;
    steps = 0;
    iterations = 0;

    // take first sample
    float dtf = consult(p,iterations);

    // if this is already a hit, report
    if ((iterations > uniforms.params.max_iterations) || (dtf < uniforms.params.dtf_limit)) {
        r = dtf;
        return true;
    }

    else {

        // keep track of nearest distance
        float nearest_dtf = uniforms.params.horizon;

        // initialize variation
        //float variation = dtf;

        // keep searching until either the ray becomes too long, or the number of steps is too high
        while ((r < uniforms.params.horizon) && (steps < uniforms.params.max_steps)) {

            // if already at or over iteration limit, take half a step back
            if (iterations > uniforms.params.max_iterations) {
                //float h = -0.5 * variation;
                float h = -0.5 * dtf;
                r += h;
                p += h * dp;
                dtf = consult(p,iterations);
                //variation = -h;
            }

            // if this is a hit, report
            if ((iterations > uniforms.params.max_iterations) || (dtf < uniforms.params.dtf_limit)) {
                return true;
            }

            // save the previous distance
            float last_dtf = dtf;

            // take step
            r += dtf;
            p += dtf * dp;

            // take next sample
            dtf = consult(p,iterations);

            // make sure the steps are always getting smaller
            //if (dtf > last_dtf + variation) {
            //    dtf = last_dtf + variation;
            //}

            // record how close we got
            nearest_dtf = min(nearest_dtf,dtf);

            steps += 1;
        }

        // no hit for whatever reason, report nearest
        r = nearest_dtf;
        return false;
    }
}
*/

// march a single ray
bool march_ray(
    in vec3 p,  // ray start
    in vec3 dp,  // march direction
    out float r,  // distance to object
    out float nearest_dtf,  // closest distance to fractal
    out uint steps,  // how many steps were taken
    out uint iterations,  // how many iterations were needed
    in float sr_per_pixel,  // solid angle per pixel
    in float r_max  // maximum distance
) {
    // initialize
    r = 0.0;
    iterations = 0;
    nearest_dtf = r_max;

    // keep searching until either the ray becomes too long, or the number of steps is too high
    for(steps = 0; (r < r_max) && (steps < uniforms.params.max_steps); steps++) {

        // sample the fractal
        float dtf = consult(p + r * dp,iterations);

        // update nearest and total distances
        nearest_dtf = min(nearest_dtf,dtf);
        r += uniforms.params.step_size * dtf;

        // if that is a hit, report
        //if ((iterations > uniforms.params.max_iterations) || (dtf < uniforms.params.dtf_limit * r / sr_per_pixel)) {
        if ((iterations > uniforms.params.max_iterations) || (dtf < uniforms.params.dtf_limit * r)) {
            return true;
        }
    }

    return false;
}

// measure depth
float measure_depth(in vec3 p,in vec3 dp,in float sr_per_pixel) {
    float r;
    float nearest_dtf;
    uint steps;
    uint iterations;
    if(march_ray(p,dp,r,nearest_dtf,steps,iterations,sr_per_pixel,uniforms.params.horizon)) {
        return r;
    }
    else {
        return uniforms.params.horizon;
    }
}

// process depth/occlusion ray
vec4 process_dosi(in vec3 p,in vec3 dp,in float sr_per_pixel) {
    float r;
    float nearest_dtf;
    uint steps;
    uint iterations;
    if (march_ray(p,dp,r,nearest_dtf,steps,iterations,sr_per_pixel,uniforms.params.horizon)) {
        float occlusion = 1.0 - clamp(float(steps * uniforms.params.step_size) / float(uniforms.params.max_steps),0.0,1.0);
        return vec4(r,occlusion,float(steps),float(iterations));
    }
    else {
        return vec4(uniforms.params.horizon,-1.0,0.0,0.0);
    }
}

// shadow attenuation
float process_shadow(in vec3 p,in vec3 dp,in float sr_per_pixel,in float r_max) {
    float r;
    float nearest_dtf;
    uint steps;
    uint iterations;
    if(march_ray(p,dp,r,nearest_dtf,steps,iterations,sr_per_pixel,r_max)) {
        return 0.0;
    }
    else {
        return clamp(uniforms.params.shadow_power.a * float(nearest_dtf / r),0.0,1.0);
    }
}
