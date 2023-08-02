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
#define CLOSEST_DISTANCE 0.0001
#define MAX_DISTANCE 100.0
#define RAY_STEP_MULTIPLIER 0.3
#define NORMAL_STEP_MULTIPLIER 0.001

// iteration parameters
#define ITERATIONS 60
#define ESCAPE_DISTANCE 20.0

// scene
#define POS vec3(4.0,2.0,20.0)

// rendering
#define OBJECT_COLOR vec3(0.1,0.2,0.9)
#define TRAP_COLOR vec3(0.8,0.8,0.8)
#define LIGHT1_POS vec3(-5,-7,-5)
#define LIGHT1_COLOR vec3(1.0,0.9,0.7)
#define LIGHT2_POS vec3(5,-4,-3)
#define LIGHT2_COLOR vec3(0.3,0.1,0.5)
#define BACKGROUND_COLOR vec3(0.1,0.1,0.2)
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

void do_iterations(inout VEC3 v,inout FLOAT dr,VEC3 c,inout FLOAT trap) {
    reciprocalz3b(v,dr,c);
    FLOAT r = length(v);
    if (r < trap) { trap = r; }
    if (r > ESCAPE_DISTANCE) return;
    rotate4d(v,dr,c);
    r = length(v);
    if (r < trap) { trap = r; }
    if (r > ESCAPE_DISTANCE) return;
    polyfoldsym(v,dr,c);
    r = length(v);
    if (r < trap) { trap = r; }
    if (r > ESCAPE_DISTANCE) return;
    amazingbox2(v,dr,c);
    r = length(v);
    if (r < trap) { trap = r; }
    if (r > ESCAPE_DISTANCE) return;
    amazingbox2(v,dr,c);
    r = length(v);
    if (r < trap) { trap = r; }
    if (r > ESCAPE_DISTANCE) return;
    amazingbox2(v,dr,c);
    r = length(v);
    if (r < trap) { trap = r; }
    if (r > ESCAPE_DISTANCE) return;
    amazingbox2(v,dr,c);
    for (int i = 0; i < ITERATIONS - 7; i++) {
        kochcube(v,dr,c);
        r = length(v);
        if (r < trap) { trap = r; }
        if (r > ESCAPE_DISTANCE) return;
    }
}

vec2 iterate_analytical(VEC3 p) {
    VEC3 c = p - POS;
    c = rotate_y(c,1.2);
    VEC3 v = c;
    FLOAT dr = 1.0;
    FLOAT trap = 1e10;
    do_iterations(v,dr,c,trap);
    FLOAT r = length(v);
    return vec2(r / abs(dr),trap);
}

vec2 sdf(vec3 p) {
    return iterate_analytical(p);
}

vec3 sdf_normal(vec3 p) {
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
    return normalize(
        k.xyy * sdf(p + NORMAL_STEP_MULTIPLIER * k.xyy).x + 
        k.yyx * sdf(p + NORMAL_STEP_MULTIPLIER * k.yyx).x + 
        k.yxy * sdf(p + NORMAL_STEP_MULTIPLIER * k.yxy).x + 
        k.xxx * sdf(p + NORMAL_STEP_MULTIPLIER * k.xxx).x
    );        
}

vec2 phong(vec3 p,vec3 light_pos) {
    vec3 l = light_pos - p;
    vec3 dp = normalize(l);
    float distance_to_light = length(l);
    vec3 n = sdf_normal(p);
    float diff = dot(n,dp);
    if (diff <= 0) {
        return vec2(0,0);
    }
    p += SHADOW_OFFSET * dp;
    float total_distance = 0.0;
    float closest_distance = MAX_DISTANCE;
    for (int steps = 0; steps < MAX_STEPS; steps++) {
        float d = float(sdf(p).x);
        p += d * dp;
        total_distance += d;
        if (total_distance > MAX_DISTANCE) {
            break;
        }
        distance_to_light -= d;
        if (distance_to_light <= 0.0) {
            break;
        }
        if (d < 0.5 * CLOSEST_DISTANCE) {
            return vec2(0,0);
        }
        closest_distance = min(closest_distance,d / total_distance);
    }
    float spec = pow(dot(normalize(dot(n,l) * n - normalize(l)),dp),128.0);
    return min(SHADOW_SHARPNESS * closest_distance,1) * vec2(diff,spec);
    //return min(SHADOW_SHARPNESS * closest_distance,1) * vec2(diff,0.0);
    //return vec2(diff,spec);
    //return vec2(diff,0.0);
}

vec4 march(vec3 p,vec3 dp) {

    float total_distance = 0.0;
    float closest_distance = MAX_DISTANCE;
    float smallest_trap = 1e10;
    int steps;
    for (steps = 0; steps < MAX_STEPS; steps++) {
        vec2 s = sdf(p);
        float d = RAY_STEP_MULTIPLIER * s.x;
        if (s.y < smallest_trap) {
            smallest_trap = s.y;
        }
        p += d * dp;
        total_distance += d;
        if (total_distance > MAX_DISTANCE) {
            float g = 1.0 - min(GLOW_SHARPNESS * closest_distance,1.0);
            g = g * g;
            g = g * g;
            vec3 pixel = (1 - g) * BACKGROUND_COLOR + g * GLOW_COLOR;
            return vec4(pixel,1.0);
        }
        if (d < CLOSEST_DISTANCE) {
            break;
        }
        closest_distance = min(closest_distance,d / total_distance);
    }

    smallest_trap = clamp(log(smallest_trap),0.0,1.0);
    vec3 pixel = smallest_trap * OBJECT_COLOR + (1.0 - smallest_trap) * TRAP_COLOR;

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
