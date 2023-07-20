#version 450

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

layout (binding = 0) readonly uniform WhatIsThisName {
    mat4 state_mvp;
    vec4 state_background_color;
};

layout (binding = 1) writeonly uniform image2D out_frame;

void main() {
    ivec2 coord = ivec2(gl_GlobalInvocationID.xy);
    imageStore(out_frame,coord,state_background_color);
}
