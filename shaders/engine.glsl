#version 450

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

layout (binding = 0) writeonly uniform image2D out_frame;

void main() {
    ivec2 coord = ivec2(gl_GlobalInvocationID.xy);
    imageStore(out_frame,coord,vec4(1,0,0,1));
}
