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

// guardings
#define MAX_STEPS 100
#define SHADOW_STEPS 100
#define CLOSEST_DISTANCE 0.002
#define MAX_DISTANCE 100.0

// iteration parameters
#define MAX_ITERATION 30
#define ESCAPE_DISTANCE 4.0

// bulb parameters
#define POWER 8

// Benesi parameters
#define STT 0.816496580927726
#define SOT 0.5773502691896258
#define SOH 0.7071067811865475
#define BENESI_SCALE 2.0
#define BENESI_OFFSET 2.0

// scene
#define BULB_POS VEC3(0.0,0.0,4.0)
#define BENESI_POS VEC3(0.0,0.0,8.0)
#define BULB_COLOR VEC3(0.6,0.3,0.1)
#define LIGHT1_POS VEC3(-5,-7,-10)
#define LIGHT1_COLOR VEC3(1.0,0.9,0.7)
#define LIGHT2_POS VEC3(5,-4,-3)
#define LIGHT2_COLOR VEC3(0.3,0.1,0.5)

// rendering
#define BACKGROUND_COLOR VEC3(0.1,0.2,0.3)
#define AMBIENT_COLOR VEC3(0.5,0.5,0.5)
#define SHADOW_OFFSET 0.01
#define SHADOW_SHARPNESS 40.0

VEC3 rotate_x(VEC3 p,FLOAT a) {
    FLOAT s = sin(a);
    FLOAT c = cos(a);
    return VEC3(p.x,c * p.y + s * p.z,-s * p.y + c * p.z);
}

VEC3 rotate_y(VEC3 p,FLOAT a) {
    FLOAT s = sin(a);
    FLOAT c = cos(a);
    return VEC3(c * p.x + s * p.z,p.y,-s * p.x + c * p.z);
}

VEC3 rotate_z(VEC3 p,FLOAT a) {
    FLOAT s = sin(a);
    FLOAT c = cos(a);
    return VEC3(c * p.x + s * p.y,-s * p.x + c * p.y,p.z);
}

FLOAT sphere(VEC3 p,VEC3 center,FLOAT radius) {
    return length(center - p) - radius;
}


// MandelBulb

FLOAT bulb(VEC3 p,VEC3 center) {
    VEC3 v = p - center;
    //v = rotate_x(v,-0.7);
    if(length(v) > 1.5) return length(v) - 1.2;
    FLOAT dr = 1.0;
    FLOAT r = 0.0;
    for (int i = 0; i < MAX_ITERATION; i++) {
        r = length(v);
        if (r > ESCAPE_DISTANCE) break;
        dr = pow(r,POWER - 1.0) * POWER * dr + 1.0;
        FLOAT theta = acos(v.z / r) * POWER;
        FLOAT phi = atan(v.y,v.x) * POWER;
        FLOAT sinTheta = sin(theta);
        v = pow(r,POWER) * VEC3(sinTheta * cos(phi),sinTheta * sin(phi),cos(theta)) + p - center;
    }
    return 0.5 * log(r) * r / dr;
}

// BenesiPine1

FLOAT benesi(VEC3 p,VEC3 center) {
    VEC3 v = p - center;
    FLOAT r = 0.0;
    for (int i = 0; i < MAX_ITERATION; i++) {
        r = length(v);
        if (r > ESCAPE_DISTANCE) break;
        FLOAT tx = (v.x * STT - v.z * SOT) * SOH;
        FLOAT z = abs(v.x * SOT + v.z * STT);
        FLOAT x = abs(tx - v.y * SOH);
        FLOAT y = abs(tx + v.y * SOH);
        tx = x * SOH + y * SOH;
        y = -x * SOH + y * SOH;
        x = tx * STT + z * SOT;
        z = -tx * SOT + z * STT;
        x = BENESI_SCALE * x + BENESI_OFFSET;
        y = BENESI_SCALE * y;
        z = BENESI_SCALE * z;
        FLOAT xt = x * x;
        FLOAT yt = y * y;
        FLOAT zt = z * z;
        FLOAT t = 2 * x / sqrt(yt + zt);
        v.x = xt - yt - zt + p.x - center.x;
        v.z = t * (yt - zt) + p.y - center.y;
        v.y = 2 * t * y * z + p.z - center.z;
    }
    return 0.5 * log(r) * r;  // dr = 1, which might work?
}

//FLOAT sdf(VEC3 p) {
//    return sphere(p,VEC3(0.0,1.0,6.0),1.0);
//}

FLOAT sdf(VEC3 p) {
    return bulb(p,BULB_POS);
    //return benesi(p,BENESI_POS);
}

VEC3 sdf_normal(VEC3 p) {
    FLOAT d = sdf(p);
    VEC3 dx = VEC3(d,0,0);
    VEC3 dy = VEC3(0,d,0);
    VEC3 dz = VEC3(0,0,d);
    return normalize(VEC3(
        sdf(p + dx),
        sdf(p + dy),
        sdf(p + dz)
    ) - VEC3(d,d,d));
}

vec2 phong(VEC3 p,VEC3 light_pos) {
    VEC3 l = light_pos - p;
    VEC3 dp = normalize(l);
    FLOAT distance_to_light = length(l);
    VEC3 n = sdf_normal(p);
    FLOAT diff = dot(n,dp);
    if (diff <= 0) {
        return VEC2(0,0);
    }
    p += SHADOW_OFFSET * dp;
    FLOAT total_distance = 0.0;
    FLOAT closest_distance = MAX_DISTANCE;
    int steps;
    for (steps = 0; steps < SHADOW_STEPS; steps++) {
        FLOAT d = sdf(p);
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
            return VEC2(0,0);
        }
        closest_distance = min(closest_distance,d / total_distance);
    }
    FLOAT spec = pow(dot(normalize(dot(n,l) * n - normalize(l)),dp),128.0);
    return min(SHADOW_SHARPNESS * closest_distance,1) * VEC2(diff,spec);
}

vec4 march(VEC3 p,VEC3 dp) {
    FLOAT distance = 0.0;
    int steps;
    for (steps = 0; steps < MAX_STEPS; steps++) {
        FLOAT d = sdf(p);
        p += d * dp;
        distance += d;
        if (distance > MAX_DISTANCE) {
            return vec4(BACKGROUND_COLOR,1.0);
        }
        if (d < CLOSEST_DISTANCE) {
            break;
        }
    }

    // ambient occlusion
    float ao = 1 - float(steps) / float(MAX_STEPS);
    vec3 pixel = ao * BULB_COLOR;

    // lighting
    vec2 ph = phong(p,LIGHT1_POS);
    pixel = (AMBIENT_COLOR + ph.x * LIGHT1_COLOR) * pixel + ph.y * LIGHT1_COLOR;

    // fog
    //FLOAT f = distance / MAX_DISTANCE;
    //pixel = (1 - f) * pixel + f * BACKGROUND_COLOR;

    return vec4(pixel,1);
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
    vec4 color = march(p,dp);

    imageStore(out_frame,coord,color);
}
