// SDF - lighting stage compute shader
// by Desmond Germans, 2023

#version 450

layout (local_size_x = 1,local_size_y = 1,local_size_z = 1) in;

#include "progress.glsl"
#include "march.glsl"
#include "brdf.glsl"
#include "render.glsl"

// 0 = uniforms, see base.glsl

layout (binding = 1,rgba32f) readonly uniform image2D dosi_image;

layout (binding = 2) writeonly uniform image2D rgba_image;

vec3 construct_normal(vec3 p,float h) {
    vec2 k = vec2(1,-1);
    uint iterations;
    return normalize(
        k.xyy * consult(p + h * k.xyy,iterations) +
        k.yxy * consult(p + h * k.yxy,iterations) +
        k.yyx * consult(p + h * k.yyx,iterations) +
        k.xxx * consult(p + h * k.xxx,iterations)
    );
}

float shadow_attenuation(vec3 p,vec3 dp,float r_max) {
    float att = 0.0;
    float r = 0.0;
    uint iterations = 0;
    float closest = r_max;
    for (uint steps = 0; (steps < uniforms.march.max_steps) && (r < r_max); steps++) {
        float de = consult(p + r * dp,iterations);
        r += de;
        if ((de < 0.1 * uniforms.march.de_stop) || (iterations > uniforms.march.max_iterations)) {
            return 0.0;
        }
        closest = min(closest,de / r);
    }
    return clamp(uniforms.render.shadow_power.a * float(closest),0.0,1.0);
}

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
#define HALF_IOD 0.003
    vec3 dir = normalize(view - origin);
    vec3 up_dir = normalize(up - origin);
    vec3 eye_axis = cross(dir,up_dir);
    if (push.eye == 0) {
        origin -= HALF_IOD * uniforms.march.scale * eye_axis;
    }
    else {
        origin += HALF_IOD * uniforms.march.scale * eye_axis;
    }

    // retrieve depth, occlusion, steps and iterations
    ivec2 cp = ivec2(int(floor(c.x)),int(floor(c.y)));
    vec4 dosi = imageLoad(dosi_image,cp);
    vec3 pixel = uniforms.render.background_color.rgb;
    if (dosi.y >= 0.0) {

        float r = dosi.x;
        float occlusion = pow(dosi.y,16.0);
        float ndist = r / (uniforms.march.scale * uniforms.march.horizon);

        // calculate incident point
        vec3 p = origin + r * dir;

        // calculate normal
        vec3 n = construct_normal(p,0.00001 * r);

        // start lighting
        //vec3 albedo = uniforms.render.albedo_color.rgb;
        vec3 albedo = sample_palette(0.1 * dosi.w).rgb;

        float metallic = 0.0;
        float roughness = 0.4;
        float reflectance = 0.8;

        // key light
        vec3 dkey_light = uniforms.render.key_light_pos.xyz - p;
        float r_max = length(dkey_light);
        dkey_light = normalize(dkey_light);
        vec3 key_shadow = pow(vec3(shadow_attenuation(p,dkey_light,r_max)),uniforms.render.shadow_power.rgb);

        // apply key light
        //vec3 key_result = uniforms.render.key_light_color.rgb * clamp(dot(n,dkey_light),0.0,1.0) * albedo;
        vec3 key_result = uniforms.render.key_light_color.rgb * brdf(n,dkey_light,-dir,metallic,roughness,albedo,reflectance);

        // sky light
        vec3 dsky_light = vec3(0.0,1.0,0.0);
        vec3 sky_shadow = pow(vec3(shadow_attenuation(p,dsky_light,uniforms.march.horizon)),uniforms.render.shadow_power.rgb);

        // apply sky light
        //vec3 sky_result = uniforms.render.sky_light_color.rgb * clamp(dot(n,dsky_light),0.0,1.0) * albedo;
        vec3 sky_result = uniforms.render.sky_light_color.rgb * brdf(n,dsky_light,-dir,metallic,roughness,albedo,reflectance);

        // apply 'global illumination'
        vec3 ambient_result = uniforms.render.ambient_light_color.rgb * albedo;

        // combine all lighting
        vec3 result = key_result * key_shadow + sky_result * sky_shadow + ambient_result * occlusion;

        // calculate fog
        float fog = clamp(16.0 * uniforms.render.background_color.a * ndist,0.0,1.0);

        // and mix with fog
        pixel = mix(result,pixel,fog);
    }

    draw_block(
        rgba_image,
        b,
        vec4(pixel,1.0)
    );
}
