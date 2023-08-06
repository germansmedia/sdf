#version 450

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

layout (binding = 0) readonly uniform WhatIsThisName {
    vec4 state_eye;
    mat4 state_world;
    vec4 state_background_color;
};

layout (binding = 1) writeonly uniform image2D out_frame;

#if 1
#define FLOAT float
#define VEC3 vec3
#define VEC4 vec4
#define MAT3 mat3
#else
#define FLOAT double
#define VEC3 dvec3
#define VEC4 dvec4
#define MAT3 dmat3
#endif

// marching parameters
#define MAX_STEPS 1000
#define CLOSEST_DISTANCE 0.1
#define MAX_DISTANCE 100.0
#define RAY_STEP_MULTIPLIER 0.1
#define NORMAL_STEP_MULTIPLIER 0.01

// iteration parameters
#define MIN_ITERATIONS 4
#define MAX_ITERATIONS 60
#define ESCAPE_DISTANCE 100

// scene
#define POS vec3(0.0,0.0,30.0)

// rendering
#define OBJECT_COLOR vec3(0.1,0.2,0.9)
#define TRAP_COLOR vec3(0.8,0.8,0.8)
#define LIGHT1_POS vec3(-5,-7,-5)
#define LIGHT1_COLOR vec3(1.0,0.9,0.7)
#define LIGHT2_POS vec3(5,-4,-3)
#define LIGHT2_COLOR vec3(0.3,0.1,0.5)
#define BACKGROUND_COLOR vec3(0.0,0.1,0.0)
#define GLOW_COLOR vec3(0.4,0.4,0.4)
#define AMBIENT_COLOR vec3(0.4,0.4,0.4)
#define SHADOW_OFFSET 0.0
#define SHADOW_SHARPNESS 40.0
#define GLOW_SHARPNESS 40.0

#include "mandelbulb.glsl"
#include "quickdudley.glsl"
#include "benesipine.glsl"
#include "cosinepow2.glsl"
#include "kochcube.glsl"
#include "mandelbox.glsl"
#include "reciprocalz3b.glsl"
#include "rotate4d.glsl"
#include "amazingbox2.glsl"
#include "polyfoldsym.glsl"

vec3 rotate_x(vec3 p,float a) {
    float s = sin(a);
    float c = cos(a);
    return vec3(p.x,c * p.y + s * p.z,-s * p.y + c * p.z);
}

vec3 rotate_y(vec3 p,float a) {
    float s = sin(a);
    float c = cos(a);
    return vec3(c * p.x + s * p.z,p.y,-s * p.x + c * p.z);
}

vec3 rotate_z(vec3 p,float a) {
    float s = sin(a);
    float c = cos(a);
    return vec3(c * p.x + s * p.y,-s * p.x + c * p.y,p.z);
}

float sphere(vec3 p,vec3 center,float radius) {
    return length(center - p) - radius;
}

FLOAT do_iterations(VEC3 c,out int i) {
    VEC3 v = c;
    FLOAT r = 0.0;
    FLOAT dr = 1.0;
    i = 0;
    /*
    reciprocalz3b(v,dr,c);
    if (i > MAX_ITERATIONS) return r / abs(dr);
    r = length(v);
    if (r > ESCAPE_DISTANCE) return r / abs(dr);
    i++;

    rotate4d(v,dr,c);
    if (i > MAX_ITERATIONS) return r / abs(dr);
    r = length(v);
    if (r > ESCAPE_DISTANCE) return r / abs(dr);
    i++;

    polyfoldsym(v,dr,c);
    if (i > MAX_ITERATIONS) return r / abs(dr);
    r = length(v);
    if (r > ESCAPE_DISTANCE) return r / abs(dr);
    i++;

    amazingbox2(v,dr,c);
    if (i > MAX_ITERATIONS) return r / abs(dr);
    r = length(v);
    if (r > ESCAPE_DISTANCE) return r / abs(dr);
    i++;

    amazingbox2(v,dr,c);
    if (i > MAX_ITERATIONS) return r / abs(dr);
    r = length(v);
    if (r > ESCAPE_DISTANCE) return r / abs(dr);
    i++;

    amazingbox2(v,dr,c);
    if (i > MAX_ITERATIONS) return r / abs(dr);
    r = length(v);
    if (r > ESCAPE_DISTANCE) return r / abs(dr);
    i++;

    amazingbox2(v,dr,c);
    if (i > MAX_ITERATIONS) return r / abs(dr);
    r = length(v);
    if (r > ESCAPE_DISTANCE) return r / abs(dr);
    i++;
    */

    kochcube(v,dr,c);
    if (i > MAX_ITERATIONS) return r / abs(dr);
    r = length(v);
    if (r > ESCAPE_DISTANCE) return r / abs(dr);
    i++;

    kochcube(v,dr,c);
    if (i > MAX_ITERATIONS) return r / abs(dr);
    r = length(v);
    if (r > ESCAPE_DISTANCE) return r / abs(dr);
    i++;

    kochcube(v,dr,c);
    if (i > MAX_ITERATIONS) return r / abs(dr);
    r = length(v);
    if (r > ESCAPE_DISTANCE) return r / abs(dr);
    i++;

    return r / abs(dr);
}

float estimate_distance(vec3 p,out int i) {
    VEC3 c = p - POS;
    //c = rotate_y(c,1.2);
    return do_iterations(c,i);
}

vec3 estimate_normal(vec3 p) {
    /*
    float d = RAY_STEP_MULTIPLIER * sdf(p);
    vec3 dx = vec3(d,0,0);
    vec3 dy = vec3(0,d,0);
    vec3 dz = vec3(0,0,d);
    return normalize(vec3(
        RAY_STEP_MULTIPLIER * sdf(p + dx),
        RAY_STEP_MULTIPLIER * sdf(p + dy),
        RAY_STEP_MULTIPLIER * sdf(p + dz)
    ) - vec3(d,d,d));
    */
    vec2 k = vec2(1,-1);
    int i;
    return normalize(
        k.xyy * estimate_distance(p + NORMAL_STEP_MULTIPLIER * k.xyy,i) + 
        k.yyx * estimate_distance(p + NORMAL_STEP_MULTIPLIER * k.yyx,i) + 
        k.yxy * estimate_distance(p + NORMAL_STEP_MULTIPLIER * k.yxy,i) + 
        k.xxx * estimate_distance(p + NORMAL_STEP_MULTIPLIER * k.xxx,i)
    );        
}

vec2 phong(vec3 p,vec3 light_pos) {
    vec3 l = light_pos - p;
    vec3 dp = normalize(l);
    float distance_to_light = length(l);
    vec3 n = estimate_normal(p);
    float diff = dot(n,dp);
    if (diff <= 0) {
        return vec2(0,0);
    }
    p += SHADOW_OFFSET * dp;
    float total_distance = 0.0;
    float closest_distance = MAX_DISTANCE;
    for (int steps = 0; steps < MAX_STEPS; steps++) {
        int i;
        float de = float(estimate_distance(p,i));
        p += de * dp;
        total_distance += de;
        if (total_distance > MAX_DISTANCE) {
            break;
        }
        distance_to_light -= de;
        if (distance_to_light <= 0.0) {
            break;
        }
        if (de < 0.5 * CLOSEST_DISTANCE) {
            return vec2(0,0);
        }
        closest_distance = min(closest_distance,de / total_distance);
    }
    float spec = pow(dot(normalize(dot(n,l) * n - normalize(l)),dp),128.0);
    return min(SHADOW_SHARPNESS * closest_distance,1) * vec2(diff,spec);
    //return min(SHADOW_SHARPNESS * closest_distance,1) * vec2(diff,0.0);
    //return vec2(diff,spec);
    //return vec2(diff,0.0);
}

vec4 march(vec3 p,vec3 dp) {

    float total_distance = 0.0;
    bool object_visible = false;
    int steps = 0;
    for (; steps < MAX_STEPS; steps++) {
        int i;
        float de = RAY_STEP_MULTIPLIER * float(estimate_distance(p,i));
        p += de * dp;
        total_distance += de;
        if (total_distance > MAX_DISTANCE) {
            // nothing found
            break;
        }
        if (de < CLOSEST_DISTANCE) {
            object_visible = true;
            break;
        }
        /*if (i > MAX_ITERATIONS) {
            // overshot object, so step back half a step
            FLOAT half_step = 0.5 * de;
            total_distance -= half_step;
            p -= half_step * dp;
            // adjust de_stop to account for distance
            de = estimate_distance(p,i);
        }*/
    }

    vec3 pixel = BACKGROUND_COLOR;
    if (object_visible) {
        pixel = OBJECT_COLOR;

        //smallest_trap = clamp(log(smallest_trap),0.0,1.0);
        //vec3 pixel = smallest_trap * OBJECT_COLOR + (1.0 - smallest_trap) * TRAP_COLOR;

        // ambient occlusion
        float ao = 1 - float(steps) / float(MAX_STEPS);
        pixel = ao * ao * pixel;

        // lighting
        vec2 ph = phong(p,LIGHT1_POS);
        pixel = (AMBIENT_COLOR + ph.x * LIGHT1_COLOR) * pixel + ph.y * LIGHT1_COLOR;

        // fog
        float f = total_distance / MAX_DISTANCE;
        f = f * f;
        pixel = (1 - f) * pixel + f * BACKGROUND_COLOR;
    }

    return vec4(pixel,1);
}

// MAIN

void main() {

    // get pixel coordinates
    ivec2 coord = ivec2(gl_GlobalInvocationID.xy);

    // starting point
    vec3 p = vec3(0.0,0.0,0.0);

    // starting direction vector
    vec3 dp = normalize(vec3(float(coord.x),float(coord.y),0.0) - state_eye.xyz);

    // DO IT!
    vec4 color = march(p,dp);

    imageStore(out_frame,coord,color);
}
