#version 450

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

layout (binding = 0) readonly uniform WhatIsThisName {
    vec4 state_eye;
    mat4 state_world;
    vec4 state_background_color;
};

layout (binding = 1) writeonly uniform image2D out_frame;

// basic types for the calculations
#define FLOAT float
#define VEC2 vec2
#define VEC3 vec3

// step size (initial for SDF)
#define STEP_SIZE 0.01

// max number of steps
#define MAX_STEPS 1000

// when to stop searching
#define CLOSEST 0.0001
#define MAX_DISTANCE 10.0

// WHEN YOU CAN'T CALCULATE THE SDF:

#define MAX_ITERATION 100
#define ESCAPE_DISTANCE 2.0

int calc_bulb(VEC3 p,VEC3 center) {
    VEC3 v = p - center;
    for(int i = 1; i <= MAX_ITERATION; i++) {
        FLOAT r = sqrt(v.x * v.x + v.y * v.y + v.z * v.z);
        FLOAT phi = atan(v.y,v.x);
        FLOAT theta = acos(v.z / r);
        FLOAT r2 = r * r;
        FLOAT r4 = r2 * r2;
        FLOAT r8 = r4 * r4;
        FLOAT s8t = sin(8 * theta);
        FLOAT nx = s8t * cos(8 * phi);
        FLOAT ny = s8t * sin(8 * phi);
        FLOAT nz = cos(8 * theta);
        v = r8 * VEC3(nx,ny,nz) + p - center;
        if(length(v) > ESCAPE_DISTANCE) {
            return i;
        }
    }
    return 0;
}

vec4 no_sdf(VEC3 start,VEC3 dp) {
    VEC3 p = start;
    FLOAT depth = 1.0;
    for (int i = 0; i < MAX_STEPS; i++) {
        p += STEP_SIZE * dp;
        depth += STEP_SIZE;
        if (depth > MAX_DISTANCE) {
            return vec4(0.1,0.1,0.1,1.0);
        }
        if (calc_bulb(p,VEC3(0.0,1.0,5.0)) == 0) {
            return fract(10.0 * depth) * vec4(1.0,0.5,0.1,1.0);
        }
    }
    return vec4(0.1,0.1,0.1,1.0);
}

// WHEN YOU CAN CALCULATE THE SDF

FLOAT sphere(VEC3 p,VEC3 center,FLOAT radius) {
    return length(center - p) - radius;
}

FLOAT calc_sdf(VEC3 p) {
    return sphere(p,VEC3(0.0,1.0,5.0),1.0);
}

VEC3 normal(VEC3 p) {
    VEC2 e = VEC2(1.0,-1.0) * 0.0005;
    return normalize(
        e.xyy * calc_sdf(p + e.xyy) +
        e.yyx * calc_sdf(p + e.yyx) +
        e.yxy * calc_sdf(p + e.yxy) +
        e.xxx * calc_sdf(p + e.xxx)
    );
}

vec4 sdf(VEC3 p,VEC3 dp) {
    FLOAT depth = 1.0;
    FLOAT d = STEP_SIZE;
    for (int i = 0; i < MAX_STEPS; i++) {
        p += d * dp;
        d = calc_sdf(p);
        depth += d;
        if (depth > MAX_DISTANCE) {
            return vec4(0.1,0.1,0.1,1.0);
        }
        if (d < CLOSEST) {
            vec3 n = normal(p);
            return vec4(0.5 + 0.5 * n.x,0.5 + 0.5 * n.y,0.5 + 0.5 * n.z,1.0);
        }
    }
    return vec4(0.1,0.1,0.1,1.0);
}

// MAIN

void main() {

    // get pixel coordinates
    ivec2 coord = ivec2(gl_GlobalInvocationID.xy);

    // starting point
    VEC3 p = VEC3(0.0,0.0,0.0);

    // starting direction vector
    VEC3 dp = normalize(VEC3(FLOAT(coord.x),FLOAT(coord.y),0.0) - state_eye.xyz);

    // DO IT!
    //vec4 color = sdf(p,dp);
    vec4 color = no_sdf(p,dp);

    imageStore(out_frame,coord,color);
}
