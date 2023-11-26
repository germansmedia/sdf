// SDF - depth/occlusion stage compute shader
// by Desmond Germans, 2023

#version 450

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

#include "progress.glsl"
#include "march.glsl"

// 0 = uniforms, see base.glsl

layout (binding = 1) writeonly uniform image2D do_image;

layout (binding = 2) writeonly uniform image2D rgba_image;

void main() {

    // get block specification
    ivec2 b;  // upper-left corner of block
    vec2 c;  // center of pixel in upper-left corner of block
    get_block_spec(b,c);

    // calculate view direction that 
    float t = 2.0 * PI * c.x / float(uniforms.view.width);
    float f = PI - PI * c.y / float(uniforms.view.height);
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
    vec2 depth_occlusion = process_depth_occlusion(origin,dir);

    // draw depth-occlusion block
    draw_block(
        do_image,
        b,
        vec4(depth_occlusion,0.0,0.0)
    );

    // draw grey RGBA preview
    draw_block(
        rgba_image,
        b,
        vec4(
            clamp(float(depth_occlusion.x) / (uniforms.march.scale * uniforms.march.horizon),0.0,1.0),
            0.5 + 0.5 * y,
            0.5 + 0.5 * z,
            //depth_occlusion.y,
            //0.0,
            1.0
        )
    );
}
