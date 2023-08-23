#version 450

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

#define MODE_OUTPUT        0
#define MODE_DEPTH         1
#define MODE_NORMAL        2
#define MODE_DEPTH_RB      3
#define MODE_ITERATIONS_RB 4
#define MODE_STEPS_RB      5
#define MODE_OCCLUSION_RB  6
#define MODE_DEBUG         7

#define PROGRESSIVE_FULL16X16 0
#define PROGRESSIVE_RIGHT8X16 1
#define PROGRESSIVE_BOTTOM8X8 2
#define PROGRESSIVE_RIGHT4X8  3
#define PROGRESSIVE_BOTTOM4X4 4
#define PROGRESSIVE_RIGHT2X4  5
#define PROGRESSIVE_BOTTOM2X2 6
#define PROGRESSIVE_RIGHT1X2  7
#define PROGRESSIVE_BOTTOM1X1 8

layout (std140,binding = 0) readonly uniform Uniforms {
    mat4 state_view;              // view matrix

    float state_fovy;             // vertical FoV
    float state_scale;            // generic scale of the operation
    float state_focus;            // distance to focus plane
    float state_aperture;         // how far apart the rays start

    uint state_mode;              // visualization mode
    uint state_max_steps;         // maximum number of ray marching steps
    uint state_max_iterations;    // maximum number of iterations
    uint tbd0;

    float state_horizon;          // furthest distance to view
    float state_escape;           // fractal iteration escape value
    float state_de_stop;          // closest approach to the fractal
    float tbd1;

    vec4 state_colors[16];        // primary color table

    vec4 state_key_light_pos;     // key light position

    vec4 state_key_light_color;   // key light color

    vec4 state_shadow_power;      // shadow power (a = sharpness)

    vec4 state_sky_light_color;   // sky light color (a = fog strength)

    vec4 state_gi_light_color;    // ambient light color

    vec4 state_background_color;  // background color;

    vec4 state_glow_color;        // glow color (a = power)
};

layout (std140,push_constant) readonly uniform PushConstants {
    vec2 state_size;
    uint state_progressive;
    uint state_offset;
};

layout (binding = 1) writeonly uniform image2D out_frame;

layout (binding = 2) readonly uniform image2D in_face;

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
#include "amazingsurf.glsl"
//#include "polyfoldsym.glsl"
#include "bulboxp2.glsl"

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


#define ITERATE(formula) \
formula(v,dr,p); \
r = length(v); if ((r >= state_escape) || (i > state_max_iterations)) return r / abs(dr); \
i++;

// Julius:
#if 1
FLOAT query_distance(VEC3 p,out uint i) {
    VEC3 v = p;
    FLOAT dr = 1.0;
    FLOAT r = length(v);
    i = 0;
    //ITERATE(rotate4d)
    //ITERATE(kochcube)
    //ITERATE(polyfoldsym)
    //ITERATE(reciprocalz3b)
    //ITERATE(rotate4d)
    //ITERATE(mandelbox)
    //ITERATE(rotate4d)
    //ITERATE(mandelbox)
    //ITERATE(rotate4d)
    //ITERATE(mandelbox)
    ITERATE(amazingsurf)
    ITERATE(amazingsurf)
    ITERATE(amazingsurf)
    ITERATE(amazingsurf)
    for (; (r < state_escape) && (i < state_max_iterations); i++) {
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
    for (i = 0; (r < state_escape) && (i < state_max_iterations); i++) {
        kochcube(v,dr,p);
        r = length(v);
    }
    return r / abs(dr);
}
#endif

FLOAT query_distance_face(VEC3,out uint i) {
    
}

VEC3 query_normal(VEC3 p,FLOAT pixel_area) {
    FLOAT h = 0.01 * state_scale;
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

float shadow_attenuation(VEC3 p,VEC3 dp,FLOAT de_stop_mul,FLOAT r_max) {
    float att = 0.0;
    FLOAT r = 0.0;
    uint i = 0;
    FLOAT closest = r_max;
    for(uint steps = 0; (steps < state_max_steps) && (r < r_max); steps++) {
        FLOAT de = query_distance(p + r * dp,i);
        r += de;
        if ((de < 0.1 * state_de_stop * de_stop_mul) || (i > state_max_iterations)) {
            return 0.0;
        }
        closest = min(closest,de / r);
    }
    return clamp(state_shadow_power.a * float(closest),0.0,1.0);
}

vec3 palette(float t) {
    return state_colors[0].rgb + state_colors[1].rgb * cos(6.28318 * (state_colors[2].rgb + state_colors[3].rgb));
}

vec3 march(VEC3 p,VEC3 dp,FLOAT pixel_area,out VEC3 n,out float occlusion,out float depth,out uint iterations,out uint steps,out bool debug) {

    // march that ray
    FLOAT r = 0.0;
    FLOAT closest = state_horizon;
    bool hit = false;
    for(steps = 0; (steps < state_max_steps) && (r < state_horizon); steps++) {
        FLOAT de = query_distance(p + r * dp,iterations);
        r += de;
        if (iterations > state_max_iterations) {
            r -= 0.5 * de;
            de = query_distance(p + r * dp,iterations);
            debug = true;
        }
        if ((de < state_de_stop * pixel_area) || (iterations > state_max_iterations)) {
            closest = 0.0;
            hit = true;
            break;
        }
        closest = min(closest,de);
    }
    
    // start with background and glow
    float glow = 1.0 - clamp(pow(float(closest),state_glow_color.a),0.0,1.0);
    vec3 pixel = state_background_color.rgb + glow * state_glow_color.rgb;

    // if fractal was hit
    if (hit) {

        // prepare final depth value
        depth = clamp(float(r) / state_horizon,0.0,1.0);

        // p is now the point of contact
        p += r * dp;

        // calculate normal at p
        n = query_normal(p,pixel_area);

        // cheap ambient occlusion
        occlusion = 1.0 - clamp(float(steps) / float(state_max_steps),0.0,1.0);

        // soft shadow from key light
        VEC3 dkey_light = state_key_light_pos.xyz - p;
        FLOAT r_max = length(dkey_light);
        dkey_light = normalize(dkey_light);
        //float key_shadow_att = shadow_attenuation(p,dkey_light,r_max,scaled_pixel_area);
        float key_shadow_att = shadow_attenuation(p,dkey_light,pixel_area,r_max);
        //float key_shadow_att = shadow_attenuation(p,dkey_light,r_max,0.001);

        // diffuse key light
        float key_light = clamp(dot(vec3(n),vec3(dkey_light)),0.0,1.0);

        // soft shadow from sky light
        //vec3 dsky_light = vec3(0.0,1.0,0.0);
        //float sky_shadow_att = shadow_attenuation(p,dsky_light,state_horizon,pixel_area * r);

        // sky light
        float sky_light = clamp(0.5 - 0.5 * float(n.y),0.0,1.0);

        // Walmart global illumination
        float gi_light = 0.1;

        // combine lighting
        vec3 diff = key_light * state_key_light_color.rgb * pow(vec3(key_shadow_att),state_shadow_power.rgb);
        //diff += sky_light * state_sky_light_color.rgb * pow(vec3(sky_shadow_att),state_shadow_power.rgb);
        diff += sky_light * state_sky_light_color.rgb * occlusion;
        diff += gi_light * state_gi_light_color.rgb * occlusion;

        // apply light and fog to this pixel
        float fog = clamp(state_background_color.a * depth,0.0,1.0);
        pixel = mix(palette(0.1 * float(iterations)) * diff,pixel,fog);
        //pixel = state_colors[iterations & 15].rgb * diff;
        //pixel = mix(vec3(0.3,0.3,0.3) * diff,pixel,fog);
    }

    return pixel;
}

void main() {

    // get start of block
    uint bx = gl_GlobalInvocationID.x << 4;
    uint by = gl_GlobalInvocationID.y << 4;

    // decode offset
    uint ofsx = state_offset & 0x0F;
    uint ofsy = state_offset >> 4;

    // get pixel coordinages for this pass
    uint px = bx + ofsx;
    uint py = by + ofsy;

    // place screen at z=1
    float f = tan(0.5 * state_fovy);
    float aspect = state_size.x / state_size.y;
    float x = -1.0 + 2.0 * (float(px) + 0.5) / state_size.x;
    float y = -1.0 + 2.0 * (float(py) + 0.5) / state_size.y;

    // transform setup by view matrix
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
    bool debug = false;

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
    pixel += 0.04 * march(p0,dp0,pixel_area,n,occlusion,depth,iterations,steps,debug);
    pixel += 0.12 * march(p1,dp1,pixel_area,n,occlusion,depth,iterations,steps,debug);
    pixel += 0.04 * march(p2,dp2,pixel_area,n,occlusion,depth,iterations,steps,debug);

    pixel += 0.12 * march(p3,dp3,pixel_area,n,occlusion,depth,iterations,steps,debug);
    pixel += 0.12 * march(p4,dp4,pixel_area,n,occlusion,depth,iterations,steps,debug);

    pixel += 0.04 * march(p5,dp5,pixel_area,n,occlusion,depth,iterations,steps,debug);
    pixel += 0.12 * march(p6,dp6,pixel_area,n,occlusion,depth,iterations,steps,debug);
    pixel += 0.04 * march(p7,dp7,pixel_area,n,occlusion,depth,iterations,steps,debug);

    pixel += 0.36 * march(p,dp,pixel_area,n,occlusion,depth,iterations,steps,debug);
    #else
    vec3 pixel = march(p,dp,pixel_area,n,occlusion,depth,iterations,steps,debug);
    #endif

    // prepare output
    vec3 c = vec3(0.0,0.0,0.0);
    switch(state_mode) {
        case MODE_OUTPUT: c = pow(pixel,vec3(1.0 / 2.2)); break;
        case MODE_DEPTH: c = vec3(1.0 - depth); break;
        case MODE_NORMAL: c = vec3(0.5) + 0.5 * vec3(n); break;
        case MODE_DEPTH_RB: c = rainbow(1.0 - depth); break;
        case MODE_ITERATIONS_RB: c = rainbow(clamp(0.05 * float(iterations),0.0,1.0)); break;
        case MODE_STEPS_RB: c = rainbow(clamp(0.05 * float(steps),0.0,1.0)); break;
        case MODE_OCCLUSION_RB: c = rainbow(1.0 - occlusion); break;
        case MODE_DEBUG: c = debug ? vec3(1.0) : vec3(0.0); break;
    }

    // and draw
    vec4 result = vec4(c,1.0);
    uint ox,oy;
    switch (state_progressive) {

        case PROGRESSIVE_FULL16X16:
            for (uint y = 0; y < 16; y++) {
                for (uint x = 0; x < 16; x++) {
                    imageStore(out_frame,ivec2(bx + x,by + y),result);
                }
            }
            break;
        
        case PROGRESSIVE_RIGHT8X16:
            ox = ofsx & 8;
            for (uint y = 0; y < 16; y++) {
                for (uint x = 0; x < 8; x++) {
                    imageStore(out_frame,ivec2(bx + ox + x,by + y),result);
                }
            }
            break;

        case PROGRESSIVE_BOTTOM8X8:
            ox = ofsx & 8;
            oy = ofsy & 8;
            for (uint y = 0; y < 8; y++) {
                for (uint x = 0; x < 8; x++) {
                    imageStore(out_frame,ivec2(bx + ox + x,by + oy + y),result);
                }
            }
            break;

        case PROGRESSIVE_RIGHT4X8:
            ox = ofsx & 12;
            oy = ofsy & 8;
            for (uint y = 0; y < 8; y++) {
                for (uint x = 0; x < 4; x++) {
                    imageStore(out_frame,ivec2(bx + ox + x,by + oy + y),result);
                }
            }
            break;

        case PROGRESSIVE_BOTTOM4X4:
            ox = ofsx & 12;
            oy = ofsy & 12;
            imageStore(out_frame,ivec2(bx + ox,    by + oy),result);
            imageStore(out_frame,ivec2(bx + ox + 1,by + oy),result);
            imageStore(out_frame,ivec2(bx + ox + 2,by + oy),result);
            imageStore(out_frame,ivec2(bx + ox + 3,by + oy),result);
            imageStore(out_frame,ivec2(bx + ox,    by + oy + 1),result);
            imageStore(out_frame,ivec2(bx + ox + 1,by + oy + 1),result);
            imageStore(out_frame,ivec2(bx + ox + 2,by + oy + 1),result);
            imageStore(out_frame,ivec2(bx + ox + 3,by + oy + 1),result);
            imageStore(out_frame,ivec2(bx + ox,    by + oy + 2),result);
            imageStore(out_frame,ivec2(bx + ox + 1,by + oy + 2),result);
            imageStore(out_frame,ivec2(bx + ox + 2,by + oy + 2),result);
            imageStore(out_frame,ivec2(bx + ox + 3,by + oy + 2),result);
            imageStore(out_frame,ivec2(bx + ox,    by + oy + 3),result);
            imageStore(out_frame,ivec2(bx + ox + 1,by + oy + 3),result);
            imageStore(out_frame,ivec2(bx + ox + 2,by + oy + 3),result);
            imageStore(out_frame,ivec2(bx + ox + 3,by + oy + 3),result);
            break;

        case PROGRESSIVE_RIGHT2X4:
            ox = ofsx & 14;
            oy = ofsy & 12;
            imageStore(out_frame,ivec2(bx + ox,    by + oy),result);
            imageStore(out_frame,ivec2(bx + ox + 1,by + oy),result);
            imageStore(out_frame,ivec2(bx + ox,    by + oy + 1),result);
            imageStore(out_frame,ivec2(bx + ox + 1,by + oy + 1),result);
            imageStore(out_frame,ivec2(bx + ox,    by + oy + 2),result);
            imageStore(out_frame,ivec2(bx + ox + 1,by + oy + 2),result);
            imageStore(out_frame,ivec2(bx + ox,    by + oy + 3),result);
            imageStore(out_frame,ivec2(bx + ox + 1,by + oy + 3),result);
            break;

        case PROGRESSIVE_BOTTOM2X2:
            ox = ofsx & 14;
            oy = ofsy & 14;
            imageStore(out_frame,ivec2(bx + ox,    by + oy),result);
            imageStore(out_frame,ivec2(bx + ox + 1,by + oy),result);
            imageStore(out_frame,ivec2(bx + ox,    by + oy + 1),result);
            imageStore(out_frame,ivec2(bx + ox + 1,by + oy + 1),result);
            break;

        case PROGRESSIVE_RIGHT1X2:
            ox = ofsx & 15;
            oy = ofsy & 14;
            imageStore(out_frame,ivec2(bx + ox,    by + oy),result);
            imageStore(out_frame,ivec2(bx + ox,    by + oy + 1),result);
            break;

        case PROGRESSIVE_BOTTOM1X1:
            imageStore(out_frame,ivec2(px,py),result);
            break;
    }
}
