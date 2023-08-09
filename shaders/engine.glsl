#version 450

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

#define MODE_OUTPUT        0
#define MODE_DEPTH         1
#define MODE_NORMAL        2
#define MODE_DEPTH_RB      3
#define MODE_ITERATIONS_RB 4
#define MODE_STEPS_RB      5
#define MODE_OCCLUSION_RB  6
#define MODE_NO_SHADOW     7

layout (std140,binding = 0) readonly uniform State {
    mat4 state_view;              // view matrix

    vec2 state_size;              // size of the output, in pixels
    float state_fovy;             // vertical FoV
    float state_scale;            // generic scale of the operation

    uint state_mode;              // visualization mode
    uint state_max_steps;         // maximum number of ray marching steps
    uint state_max_iterations;    // maximum number of iterations
    uint tbd0;

    float state_horizon;          // furthest distance to view
    float state_escape;           // fractal iteration escape value
    float state_de_stop;          // closest approach to the fractal
    uint tbd2;

    vec4 state_colors[16];        // primary color table

    vec4 state_key_light_pos;     // key light position

    vec4 state_key_light_color;   // key light color

    vec4 state_key_shadow_power;  // key shadow power (a = sharpness)

    vec4 state_sky_light_color;   // sky light color (a = fog strength)

    vec4 state_gi_light_color;    // ambient light color

    vec4 state_background_color;  // background color;

    vec4 state_glow_color;        // glow color (a = sharpness)
};

layout (binding = 1) writeonly uniform image2D out_frame;

// use capital names where f64 would be applicable
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

#include "base.glsl"

#define MANDELBOX_FOLD 1.0
#define MANDELBOX_SCALE 2.0
#define MANDELBOX_RADIUS 0.5

/*
void query_mandelbox(
    VEC3 p,        // sampling point
    out FLOAT r,   // length of iterator
    out FLOAT dr,  // some sort of derivative of iterator
    out uint i     // iteration count
) {

    VEC3 v = p;
    r = 0.0;
    dr = 1.0;
    for(i = 0; (i < state_max_iterations) && (r < state_escape); i++) {

        // MandelBox, to test if/how this works
        v = 2.0 * clamp(v,-MANDELBOX_FOLD,MANDELBOX_FOLD) - v;
        FLOAT r2 = dot(v,v);
        if (r2 < MANDELBOX_RADIUS) {
            v += v;
            dr += dr;
        }
        else if (r2 < 1.0) {
            FLOAT t = 1.0 / r2;
            v *= t;
            dr *= t;
        }
        v = MANDELBOX_SCALE * v + p;
        dr = MANDELBOX_SCALE * dr + 1.0;

        r = length(v);
    }
}

FLOAT query_distance(VEC3 p,out uint i) {
    FLOAT r = 0.0;
    FLOAT dr = 1.0;
    query_fractal(p,r,dr,i);
    return r / abs(dr);
}
*/

FLOAT query_distance(VEC3 p,out uint i) {
    VEC3 q = abs(p) - VEC3(1.0);
    FLOAT d = length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0);
    FLOAT s = 1.0;
    for(i = 0; i < state_max_iterations; i++) {
        VEC3 a = mod(p * s,2.0) - 1.0;
        s *= 3.0;
        VEC3 r = abs(1.0 - 3.0 * abs(a));
        FLOAT da = max(r.x,r.y);
        FLOAT db = max(r.y,r.z);
        FLOAT dc = max(r.z,r.x);
        float c = (min(da,min(db,dc)) - 1.0) / s;
        d = max(d,c);
    }
    return d;
}

VEC3 query_normal(VEC3 p) {
    FLOAT h = 0.001 * state_scale;
    vec2 k = vec2(1,-1);
    uint i;
    return normalize(
        k.xyy * query_distance(p + h * k.xyy,i) +
        k.yyx * query_distance(p + h * k.yyx,i) +
        k.yxy * query_distance(p + h * k.yxy,i) +
        k.xxx * query_distance(p + h * k.xxx,i)
    );
}

#define RB_PURPLE vec3(0.3,0.0,0.5)
#define RB_BLUE vec3(0.0,0.0,0.6)
#define RB_GREEN vec3(0.0,0.7,0.0)
#define RB_YELLOW vec3(0.8,0.8,0.0)
#define RB_RED vec3(1.0,0.0,0.0)

vec3 rainbow(float f) {
    uint i = min(uint(floor(4.0 * f)),3);
    float r = fract(4.0 * f);
    switch(i) {
        case 0: return mix(RB_PURPLE,RB_BLUE,r);
        case 1: return mix(RB_BLUE,RB_GREEN,r);
        case 2: return mix(RB_GREEN,RB_YELLOW,r);
        case 3: return mix(RB_YELLOW,RB_RED,r);
    }
}

void main() {

    // create screen at z=1
    float f = tan(0.5 * state_fovy);
    float aspect = state_size.x / state_size.y;
    float x = -1.0 + 2.0 * (float(gl_GlobalInvocationID.x) + 0.5) / state_size.x;
    float y = -1.0 + 2.0 * (float(gl_GlobalInvocationID.y) + 0.5) / state_size.y;
    vec3 origin = (state_view * vec4(0.0,0.0,0.0,1.0)).xyz;
    vec3 screen = (state_view * vec4(f * x,f * aspect * y,1.0,1.0)).xyz;

    // convert to f64 (if enabled)
    VEC3 p = VEC3(origin);
    VEC3 dp = VEC3(normalize(screen - origin));

    // march that ray
    FLOAT r = 0.0;
    uint iterations = 0;
    uint steps = 0;
    FLOAT closest = state_scale * state_horizon;
    bool hit = false;
    for(steps = 0; (steps < state_max_steps) && (r < state_scale * state_horizon); steps++) {
        FLOAT de = query_distance(p + r * dp,iterations);
        closest = min(closest,de);
        if (de < state_de_stop) {
            closest = 0.0;
            hit = true;
            break;
        }
        r += de;
    }

    // start with background and glow
    float glow = 1.0 - clamp(pow(closest,state_glow_color.a),0.0,1.0);
    vec3 pixel = state_background_color.rgb + glow * state_glow_color.rgb;

    // if fractal was hit
    VEC3 n = VEC3(0.0,0.0,0.0);
    float occlusion = 1.0;
    float depth = 1.0;
    if (hit) {

        // prepare final depth value
        depth = clamp(r / (state_scale * state_horizon),0.0,1.0);

        // p is now the point of contact
        p += r * dp;

        // calculate normal at p
        n = query_normal(p);

        // cheap ambient occlusion
        occlusion = 1.0 - clamp(float(steps) / float(state_max_steps),0.0,1.0);

        // soft shadow towards key light
        float shadow = 1.0;
        VEC3 dl = VEC3(state_key_light_pos) - p;
        FLOAT rl_max = length(dl);
        dl = normalize(dl);
        if (state_mode != MODE_NO_SHADOW) {
            FLOAT rl = 0.0;
            uint i = 0;
            uint steps = 0;
            FLOAT closest = state_scale * state_horizon;
            for(steps = 0; (steps < state_max_steps) && (rl < rl_max); steps++) {
                FLOAT de = query_distance(p + rl * dl,i);
                closest = min(closest,de);
                if (de < state_de_stop) {
                    closest = 0.0;
                    break;
                }
                rl += de;
            }
            shadow = clamp(state_key_shadow_power.a * closest / rl,0.0,1.0);
        }

        // diffuse key light
        float key_light = clamp(dot(n,dl),0.0,1.0);

        // sky light
        float sky_light = clamp(0.5 + 0.5 * n.y,0.0,1.0);

        // Walmart global illumination
        float gi_light = 0.1;

        // combine lighting
        vec3 diff = key_light * state_key_light_color.rgb * pow(vec3(shadow),state_key_shadow_power.rgb);
        diff += sky_light * state_sky_light_color.rgb * occlusion;
        diff += gi_light * state_gi_light_color.rgb * occlusion;

        // apply light and fog to this pixel
        float fog = clamp(state_sky_light_color.a * depth,0.0,1.0);
        pixel = mix(state_colors[iterations & 15].rgb * diff,pixel,fog);
        //pixel = state_colors[iterations & 15].rgb * diff;
    }

    // prepare output
    vec3 c = vec3(0.0,0.0,0.0);
    switch(state_mode) {
        case MODE_OUTPUT: c = pixel; break;
        case MODE_DEPTH: c = vec3(1.0 - depth); break;
        case MODE_NORMAL: c = vec3(0.5) + 0.5 * n; break;
        case MODE_DEPTH_RB: c = rainbow(1.0 - depth); break;
        case MODE_ITERATIONS_RB: c = rainbow(clamp(0.05 * float(iterations),0.0,1.0)); break;
        case MODE_STEPS_RB: c = rainbow(clamp(0.05 * float(steps),0.0,1.0)); break;
        case MODE_OCCLUSION_RB: c = rainbow(1.0 - occlusion); break;
        case MODE_NO_SHADOW: c = pixel; break;
    }

    // and draw
    imageStore(out_frame,ivec2(gl_GlobalInvocationID.xy),vec4(c,1.0));
}
