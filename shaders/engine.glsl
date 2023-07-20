#version 450

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

layout (binding = 0) readonly uniform WhatIsThisName {
    vec4 state_eye;
    mat4 state_world;
    vec4 state_background_color;
};

layout (binding = 1) writeonly uniform image2D out_frame;

#define MAX_STEPS 100
#define CLOSEST 0.0001
#define MAX_DISTANCE 10000

float scene(vec3 p) {
    vec3 sphere1_center = vec3(0.0,0.0,600.0);
    float sphere1_radius = 20.0;
    float d = length(sphere1_center - p) - sphere1_radius;

    vec3 sphere2_center = vec3(-80.0,20.0,700.0);
    float sphere2_radius = 60.0;
    d = min(d,length(sphere2_center - p) - sphere2_radius);

    vec3 sphere3_center = vec3(70.0,-60.0,400.0);
    float sphere3_radius = 40.0;
    return min(d,length(sphere3_center - p) - sphere3_radius);
}

vec3 normal(vec3 p) {
    vec2 e = vec2(1.0,-1.0) * 0.0005;
    return normalize(
        e.xyy * scene(p + e.xyy) +
        e.yyx * scene(p + e.yyx) +
        e.yxy * scene(p + e.yxy) +
        e.xxx * scene(p + e.xxx)
    );
}

void main() {

    // get pixel coordinates
    ivec2 coord = ivec2(gl_GlobalInvocationID.xy);

    // starting point
    vec3 p = vec3(0.0,0.0,0.0);
    float depth = 1.0;

    // starting direction vector
    vec3 dp = normalize(vec3(float(coord.x),float(coord.y),0.0) - state_eye.xyz);
    float d = 1.0;

    // iterate at most MAX_STEPS
    for (int i = 0; i < MAX_STEPS; i++) {

        // march
        p += d * dp;

        // sample scene
        d = scene(p);
        depth += d;

        // if too far away, write dark gray and exit
        if (depth > MAX_DISTANCE) {
            imageStore(out_frame,coord,vec4(0.1,0.1,0.1,1.0));
            return;
        }

        // if found something...
        if (d < CLOSEST) {

            // calculate normal
            vec3 n = normal(p);

            imageStore(out_frame,coord,vec4(0.5 + 0.5 * n.x,0.5 + 0.5 * n.y,0.5 + 0.5 * n.z,1.0));
            return;
        }
    }

    // if no object found, write dark gray and exit
    imageStore(out_frame,coord,vec4(0.1,0.1,0.1,1.0));
}
