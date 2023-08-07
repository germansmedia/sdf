#version 450

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

layout (binding = 0) readonly uniform State {
    mat4 view;
    vec4 refs;
    vec4 params;
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
#define MAX_STEPS 100
#define CLOSEST_DISTANCE 0.01
#define MAX_DISTANCE 100.0
#define RAY_STEP_MULTIPLIER 0.01
#define NORMAL_STEP_MULTIPLIER 0.001

// iteration parameters
#define ESCAPE_DISTANCE 10

// from MB3D: this is min(sqrt(Z_STEP_DIV),0.9)
#define DE_SUB min(sqrt(Z_STEP_DIV),0.9)

// from MB3D: this is 0.5 * max(width,height) * sqrt(Z_STEP_DIV + 0.001)
#define MCTMH04ZSD (512.0 * sqrt(Z_STEP_DIV + 0.001))

// rendering
#define OBJECT_COLOR vec3(0.5,0.4,0.2)
#define TRAP_COLOR vec3(0.8,0.8,0.8)
#define LIGHT_POS vec3(-2,-6,-5)
#define LIGHT_COLOR vec3(1.0,0.9,0.7)
#define BACKGROUND_COLOR vec3(0.3,0.4,0.5)
#define GLOW_COLOR vec3(0.4,0.4,0.4)
#define AMBIENT_COLOR vec3(0.4,0.4,0.4)
#define SHADOW_OFFSET 0.001
#define SHADOW_SHARPNESS 60.0
#define GLOW_SHARPNESS 40.0

vec3 object_color[8] = {
    vec3(0.0,0.0,0.0),
    vec3(0.0,0.0,1.0),
    vec3(0.0,1.0,0.0),
    vec3(0.0,1.0,1.0),
    vec3(1.0,0.0,0.0),
    vec3(1.0,0.0,1.0),
    vec3(1.0,1.0,0.0),
    vec3(1.0,1.0,1.0),
};

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

FLOAT do_iterations(VEC3 c,out int i,int max_iterations) {
    VEC3 v = c;
    FLOAT r = 0.0;
    FLOAT dr = 1.0;
    i = 0;

    /*
    reciprocalz3b(v,dr,c);
    r = length(v);
    if (r > ESCAPE_DISTANCE) return r / dr;
    i++;
    if (i > max_iterations) return r / dr;

    rotate4d(v,dr,c);
    r = length(v);
    if (r > ESCAPE_DISTANCE) return r / dr;
    i++;
    if (i > max_iterations) return r / dr;

    polyfoldsym(v,dr,c);
    r = length(v);
    if (r > ESCAPE_DISTANCE) return r / dr;
    i++;
    if (i > max_iterations) return r / dr;

    amazingbox2(v,dr,c);
    r = length(v);
    if (r > ESCAPE_DISTANCE) return r / dr;
    i++;
    if (i > max_iterations) return r / dr;

    amazingbox2(v,dr,c);
    r = length(v);
    if (r > ESCAPE_DISTANCE) return r / dr;
    i++;
    if (i > max_iterations) return r / dr;

    amazingbox2(v,dr,c);
    r = length(v);
    if (r > ESCAPE_DISTANCE) return r / dr;
    i++;
    if (i > max_iterations) return r / dr;

    amazingbox2(v,dr,c);
    r = length(v);
    if (r > ESCAPE_DISTANCE) return r / dr;
    i++;
    if (i > max_iterations) return r / dr;
    */

    while (i < 100) {
        //amazingbox2(v,dr,c);
        //kochcube(v,dr,c);
        mandelbox(v,dr,c);
        r = length(v);
        if (r > ESCAPE_DISTANCE) break;
        i++;
        if (i >= max_iterations) break;
    }

    return r / dr;
}

FLOAT estimate_distance(VEC3 p,out int i,int max_iterations) {
    return do_iterations(p,i,max_iterations);
}

vec3 estimate_normal(vec3 p,int max_iterations) {
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
    return vec3(normalize(
        k.xyy * estimate_distance(p + NORMAL_STEP_MULTIPLIER * k.xyy,i,max_iterations) + 
        k.yyx * estimate_distance(p + NORMAL_STEP_MULTIPLIER * k.yyx,i,max_iterations) + 
        k.yxy * estimate_distance(p + NORMAL_STEP_MULTIPLIER * k.yxy,i,max_iterations) + 
        k.xxx * estimate_distance(p + NORMAL_STEP_MULTIPLIER * k.xxx,i,max_iterations)
    ));
}

vec2 phong(vec3 p,vec3 light_pos,int max_iterations,float de_stop) {
    vec3 l = light_pos - p;
    vec3 dp = normalize(l);
    float distance_to_light = length(l);
    vec3 n = estimate_normal(p,max_iterations);
    float diff = dot(n,dp);
    if (diff < 0) {
        return vec2(0,0);
    }
    int i = 0;
    p += SHADOW_OFFSET * dp;
    float total_distance = SHADOW_OFFSET;
    float closest_distance = MAX_DISTANCE;
    float de = float(estimate_distance(p,i,max_iterations));
    while ((total_distance < MAX_DISTANCE) && (distance_to_light > 0.0)) {
        if (de < 0.5 * de_stop) {
            return vec2(0,0);
        }
        total_distance += de;
        distance_to_light -= de;
        p += de * dp;
        de = float(estimate_distance(p,i,max_iterations));
        closest_distance = min(closest_distance,de / total_distance);
    }
    float spec = pow(dot(normalize(dot(n,l) * n - normalize(l)),dp),128.0);
    return min(SHADOW_SHARPNESS * closest_distance,1) * vec2(diff,spec);
    //return min(SHADOW_SHARPNESS * closest_distance,1) * vec2(diff,0.0);
    //return vec2(diff,spec);
    //return vec2(diff,0.0);
}

vec4 march(
    vec3 ray_p,
    vec3 ray_dp,
    vec3 light_pos,
    int max_iterations,
    float z_step_div,
    float initial_de_stop,
    float de_stop_factor,
    float de_sub,
    float mctmh04zsd
) {
    VEC3 p = VEC3(ray_p);
    VEC3 dp = VEC3(ray_dp);
    FLOAT total_distance = 0.0;
    bool object_visible = false;
    int steps = 0;
    int i = 0;
    FLOAT de_stop = initial_de_stop;
    FLOAT de = estimate_distance(p,i,max_iterations);
    if ((i >= max_iterations) || (de < de_stop)) {
        object_visible = true;
    }
    else {
        FLOAT last_step_width = de * z_step_div;
        while (total_distance < MAX_DISTANCE) {
            if (i >= max_iterations) {
                FLOAT half_de = 0.1 * de;
                total_distance -= half_de;
                p -= half_de * dp;
                de = estimate_distance(p,i,max_iterations);
            }
            if ((i >= max_iterations) || (de < de_stop)) {
                object_visible = true;
                break;
            }
            else {
                /*float last_de = de;
                de = max(0.11,(de - de_sub * de_stop) * z_step_div);
                float de1 = max(0.4,de_stop) * mctmh04zsd;
                if (de1 < de) {
                    de = de1;
                }
                last_step_width = de;*/
                total_distance += de;
                p += de * dp;
                de_stop = initial_de_stop * (1.0 + total_distance * de_stop_factor);
                de = estimate_distance(p,i,max_iterations);
                /*if (de > last_de + last_step_width) {
                    de = last_de + last_step_width;
                }*/
            }
            steps += 1;
        }
    }

    vec3 pixel = BACKGROUND_COLOR;
    if (object_visible) {
        //pixel = object_color[i & 7];
        pixel = OBJECT_COLOR;

        //smallest_trap = clamp(log(smallest_trap),0.0,1.0);
        //vec3 pixel = smallest_trap * OBJECT_COLOR + (1.0 - smallest_trap) * TRAP_COLOR;

        // ambient occlusion
        float ao = 1 - clamp(float(steps) / float(MAX_STEPS),0.0,1.0);
        pixel = ao * pixel;

        // lighting
        vec2 ph = phong(vec3(p),light_pos,max_iterations,float(de_stop));
        pixel = (AMBIENT_COLOR + ph.x * LIGHT_COLOR) * pixel + ph.y * LIGHT_COLOR;

        // fog
        float f = float(total_distance) / MAX_DISTANCE;
        f = f * f;
        pixel = (1 - f) * pixel + f * BACKGROUND_COLOR;
    }

    return vec4(pixel,1);
}

// MAIN

void main() {

    // fetch params
    int max_iterations = int(params.x);
    float z_step_div = params.y;
    float de_stop = params.z;
    float de_stop_factor = params.w;
    float de_sub = min(sqrt(z_step_div),0.9);
    float mctmh04zsd = 0.5 * max(refs.x,refs.y) * sqrt(z_step_div + 0.001);

    // construct a screen at z = 1
    float f = tan(0.5 * refs.z);  // vertical FOV
    float aspect = refs.x / refs.y;
    float mx = f * aspect;
    float my = f;
    float x = -1.0 + 2.0 * (float(gl_GlobalInvocationID.x) + 0.5) / refs.x;
    float y = -1.0 + 2.0 * (float(gl_GlobalInvocationID.y) + 0.5) / refs.y;
    vec4 screen = view * vec4(mx * x,my * y,1.0,1.0);
    vec4 origin = view * vec4(0.0,0.0,0.0,1.0);
    vec3 dp = normalize(screen.xyz - origin.xyz);

    vec3 light_pos = origin.xyz;

    // DO IT!
    vec4 color = march(
        origin.xyz,
        dp,
        light_pos,

        max_iterations,
        z_step_div,
        de_stop,
        de_stop_factor,
        de_sub,
        mctmh04zsd
    );

    imageStore(out_frame,ivec2(gl_GlobalInvocationID.xy),color);
}
