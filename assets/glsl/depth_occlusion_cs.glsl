// SDF - depth/occlusion stage compute shader
// by Desmond Germans, 2023

#version 450

#include "viewconfig.glsl"
#include "progress.glsl"
#include "march.glsl"

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

layout (std140,push_constant) readonly uniform Push {
    uint eye;
    uint face;
    uint anisotropy;
    uint y_offset;
} push;

layout (std140,binding = 0) readonly uniform Uniforms {
    ViewConfig view;
    Progress progress;
    // formula params
    March march;
} uniforms;

layout (binding = 1) writeonly uniform image2D do_image;

layout (binding = 2) writeonly uniform image2D rgba_image;

void main() {

    // get block specification
    ivec2 b;  // upper-left corner of block
    vec2 c;  // center of pixel in upper-left corner of block
    get_block_spec(push.anisotropy,push.y_offset,uniforms.view,uniforms.progress,b,c);

    // calculate view direction that 
    float t = 2.0 * PI * c.x;
    float f = PI - PI * c.y;
    float x = -sin(f) * sin(t);
    float y = -cos(f);
    float z = sin(f) * cos(t);

    // transform by pose matrix
    vec3 origin = (uniforms.march.pose * vec4(0.0,0.0,0.0,1.0)).xyz;
    vec3 view = (uniforms.march.pose * vec4(x,y,z,1.0)).xyz;
    vec3 up = (uniforms.march.pose * vec4(0.0,1.0,0.0,1.0)).xyz;

    // adjust origin for eye
#define HALF_IOD 0.01
    vec3 dir = normalize(view - origin);
    vec3 up_dir = normalize(up - origin);
    vec3 eye_axis = normalize(cross(dir,up_dir));
    if (push.eye == 0) {
        origin -= HALF_IOD * uniforms.march.scale * eye_axis;
    }
    else {
        origin += HALF_IOD * uniforms.march.scale * eye_axis;
    }

    // march the ray
    vec2 depth_occlusion = process_depth_occlusion(uniforms.march,origin,dir);

    // draw depth-occlusion block
    draw_block(
        uniforms.view,
        push.anisotropy,
        uniforms.progress,
        do_image,
        b,
        vec4(depth_occlusion,0.0,0.0)
    );

    // draw grey RGBA preview
    draw_block(
        uniforms.view,
        push.anisotropy,
        uniforms.progress,
        rgba_image,
        b,
        vec4(depth_occlusion.y * vec3(0.8),1.0)
    );
}
