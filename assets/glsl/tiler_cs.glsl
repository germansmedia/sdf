// SDF - tiled progressive renderer compute shader
// by Desmond Germans, 2023

#version 450

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

// base March and Render structs and uniforms
#include "tiler_base.glsl"

// ray marching code
#include "march.glsl"

// lighting system
#include "lighting.glsl"

layout (binding = 1) writeonly uniform image2D tile_image;

void main() {

    // get total dimensions
    float total_width = float(uniforms.config.tile_count_x * uniforms.config.tile_width);
    float total_height = float(uniforms.config.tile_count_y * uniforms.config.tile_height);

    // pixel coordinates
    uint px = uniforms.config.current_tile_x * uniforms.config.tile_width + gl_GlobalInvocationID.x;
    uint py = uniforms.config.current_tile_y * uniforms.config.tile_height + gl_GlobalInvocationID.y;

    // normalize pixel coordinates
    float vx = -1.0 + 2.0 * float(px) / float(total_width);
    float vy = -1.0 + 2.0 * float(py) / float(total_height);

    // apply FOV
    vec4 tan_fov = tan(uniforms.config.fovs[push.eye]);
    float aspect = total_width / total_height;
    float x = 0.5 * aspect * (-tan_fov.x + tan_fov.y) * vx + 0.5 * (-tan_fov.x - tan_fov.y);
    float y = 0.5 * (-tan_fov.z + tan_fov.w) * vy + 0.5 * (-tan_fov.z - tan_fov.w);

    /*
    // calculate view direction (equirect)
    float t = 2.0 * PI * c.x / float(uniforms.view.width);
    float f = PI - PI * c.y / float(uniforms.view.height);
    float x = -sin(f) * sin(t);
    float y = -cos(f);
    float z = sin(f) * cos(t);
    */

    // transform by pose matrix
    vec3 origin = (uniforms.params.pose * vec4(0.0,0.0,0.0,1.0)).xyz;
    vec3 view = (uniforms.params.pose * vec4(normalize(vec3(x,y,-1.0)),1.0)).xyz;
    vec3 up = (uniforms.params.pose * vec4(0.0,1.0,0.0,1.0)).xyz;

    vec3 dir = view - origin;

    // if stereo, adjust origin for eye
    if ((uniforms.config.flags & FLAGS_STEREO) != 0) {
        vec3 up_dir = up - origin;
        vec3 eye_axis = cross(dir,up_dir);
        if (push.eye == 0) {
            origin -= 0.5 * uniforms.params.iod * uniforms.params.scale * eye_axis;
        }
        else {
            origin += 0.5 * uniforms.params.iod * uniforms.params.scale * eye_axis;
        }
    }

    // march the ray
    vec4 dosi = process_dosi(origin,dir);

    // process the lighting
    vec3 pixel = process_lighting(dosi,origin,dir);

    // and draw
    imageStore(tile_image,ivec2(gl_GlobalInvocationID.x,gl_GlobalInvocationID.y),vec4(pixel,1.0));
}
