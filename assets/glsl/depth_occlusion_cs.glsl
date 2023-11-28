// SDF - depth/occlusion stage compute shader
// by Desmond Germans, 2023

#version 450

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

#include "progress.glsl"
#include "march.glsl"

// 0 = uniforms, see base.glsl

layout (binding = 1) writeonly uniform image2D dosi_image;

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
    vec3 view = (uniforms.march.pose * vec4(normalize(vec3(x,y,z)),1.0)).xyz;
    vec3 up = (uniforms.march.pose * vec4(0.0,1.0,0.0,1.0)).xyz;

    // adjust origin for eye
#define HALF_IOD 0.001
    vec3 dir = view - origin;
    vec3 up_dir = up - origin;
    vec3 eye_axis = cross(dir,up_dir);
    if (push.eye == 0) {
        origin -= HALF_IOD * uniforms.march.scale * eye_axis;
    }
    else {
        origin += HALF_IOD * uniforms.march.scale * eye_axis;
    }

    // march the ray
    vec4 dosi = process_dosi(origin,dir);

    // draw depth-occlusion-steps-iterations block
    draw_block(dosi_image,b,dosi);

    // draw grey RGBA preview
    float depth = 1.0 - clamp(dosi.x / (uniforms.march.scale * uniforms.march.horizon),0.0,1.0);
    float occlusion = dosi.y;
    vec4 color = uniforms.render.background_color;
    if (occlusion > 0.0) {
        vec3 albedo = color_scheme(0.04 * dosi.w);
        float fog = clamp(16.0 * uniforms.render.background_color.a * dosi.x / (uniforms.march.scale * uniforms.march.horizon),0.0,1.0);
        color = vec4(mix(pow(occlusion,16.0) * albedo,uniforms.render.background_color.rgb,fog),1.0);
    }
    draw_block(rgba_image,b,color);
}
