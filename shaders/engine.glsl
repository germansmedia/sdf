#version 450

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

layout (binding = 0) readonly uniform WhatIsThisName {
    vec4 state_eye;
    mat4 state_world;
    vec4 state_background_color;
};

layout (binding = 1) writeonly uniform image2D out_frame;

void main() {
    ivec2 coord = ivec2(gl_GlobalInvocationID.xy);
    vec4 ray = normalize(state_eye + vec4(float(coord.x),float(coord.y),0.0,0.0));
    vec4 color = vec4(0.5 + 0.5 * ray.x,0.5 + 0.5 * ray.y,0.0,1.0);
    imageStore(out_frame,coord,color);
}
