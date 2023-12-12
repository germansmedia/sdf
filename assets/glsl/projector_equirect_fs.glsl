// SDF - equirectangular draw fragment shader
// by Desmond Germans, 2023

#version 450

#define PI 3.1415927

layout (location = 0) in vec3 direction;

layout (location = 0) out vec4 pixel;

layout (binding = 1) uniform sampler2D image;

void main() {

    // get interpolated viewing direction
    vec3 dir = normalize(direction);

    // convert to equirectangular texture coordinate
    float u = 0.5 + 0.5 * atan(dir.x,-dir.z) / PI;
    float v = acos(dir.y) / PI;

    // lookup
    pixel = vec4(texture(image,vec2(u,v)).rgb,1.0);
}
