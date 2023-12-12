// SDF - lighting stage compute shader
// by Desmond Germans, 2023

#version 450

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

// base March and Render structs, uniforms and push constants
#include "viewer_base.glsl"

// ray marching code
#include "march.glsl"

// lighting system
#include "lighting.glsl"

layout (binding = 1,rgba32f) readonly uniform image2D dosi_image;

layout (binding = 2) writeonly uniform image2D rgba_image;

void main() {

    // get block specification
    ivec2 b;  // upper-left corner of block
    vec2 c;  // center of pixel in upper-left corner of block
    get_block_spec(b,c);

    // calculate view direction that 
    float t = 2.0 * PI * c.x / float(uniforms.config.width);
    float f = PI - PI * c.y / float(uniforms.config.height);
    float x = -sin(f) * sin(t);
    float y = -cos(f);
    float z = sin(f) * cos(t);

    // transform by pose matrix
    vec3 origin = (uniforms.march.pose * vec4(0.0,0.0,0.0,1.0)).xyz;
    vec3 view = (uniforms.march.pose * vec4(normalize(vec3(x,y,z)),1.0)).xyz;
    vec3 up = (uniforms.march.pose * vec4(0.0,1.0,0.0,1.0)).xyz;

    // adjust origin for eye
    vec3 dir = normalize(view - origin);
    vec3 up_dir = normalize(up - origin);
    vec3 eye_axis = cross(dir,up_dir);
    if (push.eye == 0) {
        origin -= 0.5 * uniforms.march.iod * uniforms.march.scale * eye_axis;
    }
    else {
        origin += 0.5 * uniforms.march.iod * uniforms.march.scale * eye_axis;
    }

    // retrieve depth, occlusion, steps and iterations
    ivec2 cp = ivec2(int(floor(c.x)),int(floor(c.y)));
    vec4 dosi = imageLoad(dosi_image,cp);

    // process lighting
    vec3 pixel = process_lighting(dosi,origin,dir);

    // and draw
    draw_block(
        rgba_image,
        b,
        vec4(pixel,1.0)
    );
}
