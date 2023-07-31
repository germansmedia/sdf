#version 450

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

layout (binding = 0) readonly uniform WhatIsThisName {
    vec4 state_eye;
    mat4 state_world;
    vec4 state_background_color;
};

layout (binding = 1) writeonly uniform image2D out_frame;

// precision
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
#define SHADOW_STEPS 1000
#define CLOSEST_DISTANCE 0.0005
#define MAX_DISTANCE 100.0
#define RAY_STEP_MULTIPLIER 0.1

// iteration parameters
#define MAX_ITERATION 7
#define ESCAPE_DISTANCE 5.0

// Benesi parameters
#define STT 0.816496580927726
#define SOT 0.5773502691896258
#define SOH 0.7071067811865475
#define BENESI_SCALE 2.0
#define BENESI_OFFSET 2.0

// CosinePow2 parameters
#define Z_MULTIPLIER 1.0

// Koch parameters
#define KOCH_STRETCH 1.0
#define KOCH_POST_SCALE 1.0
#define KOCH_ADD VEC3(0.1,0.2,0.4)
#define KOCH_FOLD 1.0
#define SIN60 0.866025404
#define COS60 0.5
#define SIN20 0.342020143
#define COS20 0.939692621
#define SIN40 0.64278761
#define COS40 0.766044443
//#define KOCH_ROTATION MAT3(1.0,0.0,0.0, 0.0,1.0,0.0, 0.0,0.0,1.0)
//#define KOCH_ROTATION MAT3(COS20,0.0,SIN20, 0.0,1.0,0.0, -SIN20,0.0,COS20)
#define KOCH_ROTATION MAT3(1.0,0.0,0.0, 0.0,COS60,SIN60, 0.0,-SIN60,COS60)

// scene
#define POS vec3(0.0,0.0,7.0)
#define BULB_COLOR vec3(0.7,0.6,0.1)
#define LIGHT1_POS vec3(-5,-7,-5)
#define LIGHT1_COLOR vec3(1.0,0.9,0.7)
#define LIGHT2_POS vec3(5,-4,-3)
#define LIGHT2_COLOR vec3(0.3,0.1,0.5)

// rendering
#define BACKGROUND_COLOR vec3(0.1,0.1,0.2)
#define AMBIENT_COLOR vec3(0.4,0.4,0.4)
#define SHADOW_OFFSET 0.0
#define SHADOW_SHARPNESS 40.0

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

// MandelBulb

/*
float bulb(vec3 p,vec3 center) {
    VEC3 c = p - center;
    VEC33 v = c;
    if(length(v) > 1.5) return float(length(v) - 1.2);
    FLOAT dr = 1.0;
    FLOAT r = length(v);
    for (int i = 0; i < MAX_ITERATION; i++) {
        r = length(v);
        if (r > ESCAPE_DISTANCE) break;
        FLOAT r2 = r * r;
        FLOAT r4 = r2 * r2;
        FLOAT r7 = r * r2 * r4;
        FLOAT r8 = r4 * r4;
        dr = 8.0 * r7 * dr + 1.0;
        FLOAT theta = 8.0 * acos(v.z / r);
        FLOAT phi = 8.0 * atan(v.y,v.x);
        FLOAT sinTheta = sin(theta);
        v = r8 * VEC3(sinTheta * cos(phi),sinTheta * sin(phi),cos(theta)) + c;
    }
    return 0.5 * log(float(r)) * float(r) / float(dr);
}
*/

// BenesiPine1

float benesi(vec3 p,vec3 center) {
    VEC3 c = p - center;
    VEC3 v = c;
    FLOAT r = length(v);
    FLOAT dr = 1.0;
    for (int i = 0; i < MAX_ITERATION; i++) {
        r = length(v);
        if (r > ESCAPE_DISTANCE) break;
        dr = 2.0 * dr * r;
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
        v = VEC3(
            xt - yt - zt,
            2 * t * y * t * (yt - zt),
            t * (yt - zt)
        ) + c;
    }
    return 0.5 * log(float(r)) * float(r) / float(dr);  // would this work?
}

// CosinePow2

float cosine_pow2(vec3 p,vec3 center) {
    VEC3 c = p - center;
    VEC3 v = c;
    FLOAT r = length(v);
    FLOAT dr = 1.0;
    for (int i = 0; i < MAX_ITERATION; i++) {
        r = length(v);
        if (r > ESCAPE_DISTANCE) break;
        dr = 2.0 * dr * r;
        FLOAT q = 2 * v.z / sqrt(v.x * v.x + v.y * v.y);
        v = VEC3(
            (v.y * v.y - v.x * v.x) * q,
            2 * v.x * v.y * q,
            (v.z * v.z - v.x * v.x - v.y * v.y) * Z_MULTIPLIER
        ) + c;
    }
    return 0.5 * log(float(r)) * float(r) / float(dr);
}

// QuickDudley

float quick_dudley(vec3 p,vec3 center) {
    VEC3 c = p - center;
    VEC3 v = c;
    FLOAT r = length(v);
    FLOAT dr = 1.0;
    for (int i = 0; i < MAX_ITERATION; i++) {
        r = length(v);
        if (r > ESCAPE_DISTANCE) break;
        dr = 2.0 * dr * r;
        v = VEC3(
            v.x * v.x - 2.0 * v.z * v.y,
            v.z * v.z + 2.0 * v.y * v.x,
            v.y * v.y + 2.0 * v.x * v.z
        ) + c;
    }
    return 0.5 * log(float(r)) * float(r) / float(dr);
}

// Koch Cube

float koch_cube(vec3 p,vec3 center,out int iter) {
    VEC3 c = rotate_y(rotate_x(p - center,0.2),0.1);
    //VEC3 c = p - center;
    VEC3 v = c;
    FLOAT r = length(v);
    FLOAT w = 1.0;
    int i;
    for (i = 0; i < MAX_ITERATION; i++) {
        r = length(v);
        if (r > ESCAPE_DISTANCE) break;
        v = 3 * abs(v);
        w = 3 * KOCH_POST_SCALE * w / KOCH_STRETCH;
        if (v.y > v.x) {
            v = v.yxz;
        }
        if (v.z > v.x) {
            v = v.zyx;
        }
        if (v.z > v.y) {
            v = v.xzy;
        }
        v = KOCH_ROTATION * (v + KOCH_ADD);
        v.z = KOCH_FOLD - abs(KOCH_FOLD - v.z);
        FLOAT c = v.x - (3 - KOCH_STRETCH);
        FLOAT d = v.x - (3 + KOCH_STRETCH);
        if (c < v.y) {
            v.x = c;
            v.y = v.y - (3 - KOCH_STRETCH);
        }
        else if (d > v.y) {
            v.x = d;
        }
        else {
            v.x = v.y;
            v.y = d;
        }
        v = KOCH_POST_SCALE * VEC3(v.x / KOCH_STRETCH,v.y / KOCH_STRETCH,v.z);
    }
    iter = i;
    return float(r / abs(w));
}

//float sdf(vec3 p) {
//    return sphere(p,vec3(0.0,1.0,6.0),1.0);
//}

float sdf(vec3 p,out int iter) {
    //return bulb(p,POS);
    //return benesi(p,POS);
    //return cosine_pow2(p,POS);
    //return quick_dudley(p,POS);
    return koch_cube(p,POS,iter);
}

vec3 sdf_normal(vec3 p) {
    int iter = 0;
    float d = RAY_STEP_MULTIPLIER * sdf(p,iter);
    vec3 dx = vec3(d,0,0);
    vec3 dy = vec3(0,d,0);
    vec3 dz = vec3(0,0,d);
    return normalize(vec3(
        RAY_STEP_MULTIPLIER * sdf(p + dx,iter),
        RAY_STEP_MULTIPLIER * sdf(p + dy,iter),
        RAY_STEP_MULTIPLIER * sdf(p + dz,iter)
    ) - vec3(d,d,d));
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
    int steps,iter;
    for (steps = 0; steps < SHADOW_STEPS; steps++) {
        float d = sdf(p,iter);
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
}

vec4 march(vec3 p,vec3 dp) {

    float distance = 0.0;
    int steps,iter = 0,max_iter = MAX_ITERATION;
    for (steps = 0; steps < MAX_STEPS; steps++) {
        float d = RAY_STEP_MULTIPLIER * sdf(p,iter);
        if (iter < max_iter) {
            max_iter = iter;
        }
        p += d * dp;
        distance += d;
        if (distance > MAX_DISTANCE) {
            return vec4(BACKGROUND_COLOR,1.0);
        }
        if (d < CLOSEST_DISTANCE) {
            break;
        }
    }

    vec3 color = BULB_COLOR;
    if (max_iter > 4) {
        color = vec3(0.2,0.0,0.0);
    }

    // ambient occlusion
    float ao = 1 - float(steps) / float(MAX_STEPS);
    vec3 pixel = ao * ao * color;

    // lighting
    vec2 ph = phong(p,LIGHT1_POS);
    pixel = (AMBIENT_COLOR + ph.x * LIGHT1_COLOR) * pixel + ph.y * LIGHT1_COLOR;

    // fog
    float f = distance / MAX_DISTANCE;
    pixel = (1 - f * f) * pixel + f * f * BACKGROUND_COLOR;

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
