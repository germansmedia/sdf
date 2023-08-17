#version 450

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

#define MODE_OUTPUT            0
#define MODE_DEPTH             1
#define MODE_NORMAL            2
#define MODE_DEPTH_RB          3
#define MODE_ITERATIONS_RB     4
#define MODE_STEPS_RB          5
#define MODE_OCCLUSION_RB      6

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
    float state_focus;

    float state_aperture;
    float tbd1;
    float tbd2;
    float tbd3;

    vec4 state_colors[16];        // primary color table

    vec4 state_key_light_pos;     // key light position

    vec4 state_key_light_color;   // key light color

    vec4 state_shadow_power;      // shadow power (a = sharpness)

    vec4 state_sky_light_color;   // sky light color (a = fog strength)

    vec4 state_gi_light_color;    // ambient light color

    vec4 state_background_color;  // background color;

    vec4 state_glow_color;        // glow color (a = power)
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
#include "menger3.glsl"
#include "mandelbox.glsl"
#include "kochcube.glsl"
#include "sierphilbert.glsl"
#include "reciprocalz3b.glsl"
#include "rotate4d.glsl"
#include "amazingbox2.glsl"
#include "polyfoldsym.glsl"

#if 0
// MandelBulb
FLOAT query_distance(VEC3 p,out uint i) {
    VEC3 v = p;
    FLOAT dr = 1.0;
    FLOAT r = length(v);
    for (i = 0; (i < state_max_iterations) && (r < state_escape); i++) {
        FLOAT r2 = r * r;
        FLOAT r4 = r2 * r2;
        FLOAT r7 = r * r2 * r4;
        FLOAT r8 = r4 * r4;
        dr = 8.0 * r7 * dr + 1.0;
        FLOAT theta = 8.0 * acos(v.z / r);
        FLOAT phi = 8.0 * atan(v.y,v.x);
        FLOAT sinTheta = sin(theta);
        v = r8 * VEC3(sinTheta * cos(phi),sinTheta * sin(phi),cos(theta)) + p;
        r = length(v);
    }
    return 0.5 * log(r) * r / dr;  // this is different from 'regular' ones
}
#endif

// Julius:
#if 1
FLOAT query_distance(VEC3 p,out uint i) {
    VEC3 v = p;
    FLOAT dr = 1.0;
    FLOAT r = length(v);
    i = 0;
    reciprocalz3b(v,dr,p);
    r = length(v); if (r >= state_escape) return r / abs(dr);
    i++;
    rotate4d(v,dr,p);
    r = length(v); if (r >= state_escape) return r / abs(dr);
    i++;
    polyfoldsym(v,dr,p);
    r = length(v); if (r >= state_escape) return r / abs(dr);
    i++;
    amazingbox2(v,dr,p);
    r = length(v); if (r >= state_escape) return r / abs(dr);
    i++;
    amazingbox2(v,dr,p);
    r = length(v); if (r >= state_escape) return r / abs(dr);
    i++;
    amazingbox2(v,dr,p);
    r = length(v); if (r >= state_escape) return r / abs(dr);
    i++;
    amazingbox2(v,dr,p);
    r = length(v); if (r >= state_escape) return r / abs(dr);
    i++;
    for (; (i < state_max_iterations) && (r < state_escape); i++) {
        kochcube(v,dr,p);
        r = length(v);
    }
    return r / abs(dr);
}
#else
FLOAT query_distance(VEC3 p,out uint i) {
    VEC3 v = p;
    FLOAT dr = 1.0;
    FLOAT r = length(v);
    for (i = 0; (i < state_max_iterations) && (r < state_escape); i++) {
        kochcube(v,dr,p);
        r = length(v);
    }
    return r / abs(dr);
}
#endif

VEC3 query_normal(VEC3 p,float pixel_area) {
    FLOAT h = pixel_area * state_scale;
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
    uint i = uint(floor(4.0 * f));
    float r = fract(4.0 * f);
    switch(i) {
        case 0: return mix(RB_PURPLE,RB_BLUE,r);
        case 1: return mix(RB_BLUE,RB_GREEN,r);
        case 2: return mix(RB_GREEN,RB_YELLOW,r);
        case 3: return mix(RB_YELLOW,RB_RED,r);
        case 4: return RB_RED;
    }
}

float shadow_attenuation(vec3 p,vec3 dp,float de_stop_mul,float r_max) {
    float att = 0.0;
    float r = 0.0;
    uint i = 0;
    float closest = r_max;
    for(uint steps = 0; (steps < state_max_steps) && (r < r_max); steps++) {
        float de = query_distance(p + r * dp,i);
        if (de < 0.1 * state_de_stop * de_stop_mul) {
            return 0.0;
        }
        closest = min(closest,de / r);
        r += de;
    }
    return clamp(state_shadow_power.a * closest,0.0,1.0);
}

vec3 march(VEC3 p,VEC3 dp,float pixel_area,out VEC3 n,out float occlusion,out float depth,out uint iterations,out uint steps) {

    // march that ray
    FLOAT r = 0.0;
    FLOAT closest = state_scale * state_horizon;
    bool hit = false;
    for(steps = 0; (steps < state_max_steps) && (r < state_scale * state_horizon); steps++) {
        FLOAT de = query_distance(p + r * dp,iterations);
        closest = min(closest,de);
        //if (de < state_de_stop * pixel_area) {
        if (de < state_de_stop * pixel_area) {
        //if (de < 0.001 * state_de_stop) {
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
    if (hit) {

        // prepare final depth value
        depth = clamp(r / (state_scale * state_horizon),0.0,1.0);

        // p is now the point of contact
        p += r * dp;

        // calculate normal at p
        n = query_normal(p,pixel_area);

        // cheap ambient occlusion
        occlusion = 1.0 - clamp(float(steps) / float(state_max_steps),0.0,1.0);

        // soft shadow from key light
        vec3 dkey_light = state_key_light_pos.xyz - p;
        float r_max = length(dkey_light);
        dkey_light = normalize(dkey_light);
        //float key_shadow_att = shadow_attenuation(p,dkey_light,r_max,scaled_pixel_area);
        float key_shadow_att = shadow_attenuation(p,dkey_light,pixel_area,r_max);
        //float key_shadow_att = shadow_attenuation(p,dkey_light,r_max,0.001);

        // diffuse key light
        float key_light = clamp(dot(n,dkey_light),0.0,1.0);

        // soft shadow from sky light
        //vec3 dsky_light = vec3(0.0,1.0,0.0);
        //float sky_shadow_att = shadow_attenuation(p,dsky_light,state_scale * state_horizon,pixel_area * r);

        // sky light
        float sky_light = clamp(0.5 - 0.5 * n.y,0.0,1.0);

        // Walmart global illumination
        float gi_light = 0.1;

        // combine lighting
        vec3 diff = key_light * state_key_light_color.rgb * pow(vec3(key_shadow_att),state_shadow_power.rgb);
        //diff += sky_light * state_sky_light_color.rgb * pow(vec3(sky_shadow_att),state_shadow_power.rgb);
        diff += sky_light * state_sky_light_color.rgb * occlusion;
        diff += gi_light * state_gi_light_color.rgb * occlusion;

        // apply light and fog to this pixel
        float fog = clamp(state_background_color.a * depth,0.0,1.0);
        pixel = mix(state_colors[iterations & 15].rgb * diff,pixel,fog);
        //pixel = state_colors[iterations & 15].rgb * diff;
        //pixel = mix(vec3(0.3,0.3,0.3) * diff,pixel,fog);
    }

    return pixel;
}

void main() {

    // create screen at z=1
    float f = tan(0.5 * state_fovy);
    float aspect = state_size.x / state_size.y;
    float x = -1.0 + 2.0 * (float(gl_GlobalInvocationID.x) + 0.5) / state_size.x;
    float y = -1.0 + 2.0 * (float(gl_GlobalInvocationID.y) + 0.5) / state_size.y;
    vec3 origin = (state_view * vec4(0.0,0.0,0.0,1.0)).xyz;
    vec3 screen = (state_view * vec4(f * aspect * x,f * y,1.0,1.0)).xyz;

    // calculate the area  of a single pixel
    float fpp = 2.0 * f / state_size.y;
    float pixel_area = fpp * fpp;

    // convert to f64 (if enabled)
    VEC3 p = VEC3(origin);
    VEC3 dp = VEC3(normalize(screen - origin));

    // get ready...
    VEC3 n = VEC3(0.0,0.0,0.0);
    float occlusion = 1.0;
    float depth = 1.0;
    uint iterations = 0;
    uint steps = 0;

    // depth-of-field
    #if 0
    vec3 screen_dx = (state_view * vec4(f * aspect,0.0,1.0,1.0)).xyz;
    vec3 screen_dy = (state_view * vec4(0.0,f,1.0,1.0)).xyz;

    VEC3 dpx = state_scale * state_aperture * VEC3(normalize(screen_dx));
    VEC3 dpy = state_scale * state_aperture * VEC3(normalize(screen_dy));

    VEC3 pf = p + state_scale * state_focus * dp;

    VEC3 p0 = p - dpx - dpy;
    VEC3 dp0 = normalize(pf - p0);
    VEC3 p1 = p - dpy;
    VEC3 dp1 = normalize(pf - p1);
    VEC3 p2 = p + dpx - dpy;
    VEC3 dp2 = normalize(pf - p2);

    VEC3 p3 = p - dpx;
    VEC3 dp3 = normalize(pf - p3);
    VEC3 p4 = p + dpx;
    VEC3 dp4 = normalize(pf - p4);

    VEC3 p5 = p - dpx + dpy;
    VEC3 dp5 = normalize(pf - p5);
    VEC3 p6 = p + dpy;
    VEC3 dp6 = normalize(pf - p6);
    VEC3 p7 = p + dpx + dpy;
    VEC3 dp7 = normalize(pf - p7);

    vec3 pixel = vec3(0.0,0.0,0.0);
    pixel += 0.04 * march(p0,dp0,pixel_area,n,occlusion,depth,iterations,steps);
    pixel += 0.12 * march(p1,dp1,pixel_area,n,occlusion,depth,iterations,steps);
    pixel += 0.04 * march(p2,dp2,pixel_area,n,occlusion,depth,iterations,steps);

    pixel += 0.12 * march(p3,dp3,pixel_area,n,occlusion,depth,iterations,steps);
    pixel += 0.12 * march(p4,dp4,pixel_area,n,occlusion,depth,iterations,steps);

    pixel += 0.04 * march(p5,dp5,pixel_area,n,occlusion,depth,iterations,steps);
    pixel += 0.12 * march(p6,dp6,pixel_area,n,occlusion,depth,iterations,steps);
    pixel += 0.04 * march(p7,dp7,pixel_area,n,occlusion,depth,iterations,steps);

    pixel += 0.36 * march(p,dp,pixel_area,n,occlusion,depth,iterations,steps);
    #else
    vec3 pixel = march(p,dp,pixel_area,n,occlusion,depth,iterations,steps);
    #endif

    // prepare output
    vec3 c = vec3(0.0,0.0,0.0);
    switch(state_mode) {
        case MODE_OUTPUT: c = pow(pixel,vec3(1.0 / 2.2)); break;
        case MODE_DEPTH: c = vec3(1.0 - depth); break;
        case MODE_NORMAL: c = vec3(0.5) + 0.5 * n; break;
        case MODE_DEPTH_RB: c = rainbow(1.0 - depth); break;
        case MODE_ITERATIONS_RB: c = rainbow(clamp(0.05 * float(iterations),0.0,1.0)); break;
        case MODE_STEPS_RB: c = rainbow(clamp(0.05 * float(steps),0.0,1.0)); break;
        case MODE_OCCLUSION_RB: c = rainbow(1.0 - occlusion); break;
    }

    // and draw
    imageStore(out_frame,ivec2(gl_GlobalInvocationID.xy),vec4(c,1.0));
}
