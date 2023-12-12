// SDF - depth/occlusion stage compute shader
// by Desmond Germans, 2023

#version 450

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

// base March and Render structs, uniforms and push constants
#include "viewer_base.glsl"

// ray marching code
#include "march.glsl"

// palette lookup for RGBA preview
#include "palette.glsl"

// output depth/occlusion/steps/iterations image
layout (binding = 1) writeonly uniform image2D dosi_image;

// output RGBA image
layout (binding = 2) writeonly uniform image2D rgba_image;

void main() {

    // get block specification
    ivec2 b;  // upper-left corner of block
    vec2 c;  // center of pixel in upper-left corner of block
    get_block_spec(b,c);

    // calculate view direction
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
    vec3 dir = view - origin;
    vec3 up_dir = up - origin;
    vec3 eye_axis = cross(dir,up_dir);
    if (push.eye == 0) {
        origin -= 0.5 * uniforms.march.iod * uniforms.march.scale * eye_axis;
    }
    else {
        origin += 0.5 * uniforms.march.iod * uniforms.march.scale * eye_axis;
    }

    // march the ray
    vec4 dosi = process_dosi(origin,dir);

    // draw depth-occlusion-steps-iterations block
    draw_block(dosi_image,b,dosi);

    // draw RGBA preview without lights
    vec3 pixel = uniforms.render.background_color.rgb;
    if (dosi.y > 0.0) {
        float r = dosi.x;
        float ndist = r / (uniforms.march.scale * uniforms.march.horizon);
        float occlusion = pow(dosi.y,16.0);
        vec3 albedo = sample_palette(0.1 * dosi.w).rgb;
        vec3 ambient_result = uniforms.render.ambient_light_color.rgb * albedo;
        vec3 result = ambient_result * occlusion;
        float fog = clamp(16.0 * uniforms.render.background_color.a * ndist,0.0,1.0);
        pixel = mix(result,pixel,fog);
    }
    draw_block(rgba_image,b,vec4(pixel,1.0));
}
