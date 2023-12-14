// SDF - lighting system
// by Desmond Germans, 2023

// palette lookup for albedo
#include "palette.glsl"

// BRDF calculation
#include "brdf.glsl"

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

vec3 process_lighting(in vec4 dosi,in vec3 origin,in vec3 dir,in float sr_per_pixel) {

    vec3 pixel = uniforms.params.background_color.rgb;
    if (dosi.y >= 0.0) {

        float r = dosi.x;
        float occlusion = pow(dosi.y,4.0);
        float ndist = r / (uniforms.params.scale * uniforms.params.horizon);

        // calculate incident point
        vec3 p = origin + r * dir;

        // calculate normal
        vec3 n = construct_normal(p,0.0001 * r);

        // start lighting
        vec3 albedo = sample_palette(dosi.w).rgb;
        //vec3 albedo = sample_palette(0.0).rgb;

        float metallic = 0.0;
        float roughness = 0.7;
        float reflectance = 1.0;

        // key light
        vec3 dkey_light = uniforms.params.key_light_pos.xyz - p;
        float r_max = length(dkey_light);
        dkey_light = normalize(dkey_light);
        vec3 key_shadow = pow(vec3(process_shadow(p,dkey_light,sr_per_pixel,r_max)),uniforms.params.shadow_power.rgb);

        // apply key light
        vec3 key_result = uniforms.params.key_light_color.rgb * brdf(n,dkey_light,-dir,metallic,roughness,albedo,reflectance);

        // sky light
        vec3 dsky_light = vec3(0.0,1.0,0.0);
        vec3 sky_shadow = pow(vec3(process_shadow(p,dsky_light,sr_per_pixel,uniforms.params.horizon)),uniforms.params.shadow_power.rgb);

        // apply sky light
        vec3 sky_result = uniforms.params.sky_light_color.rgb * brdf(n,dsky_light,-dir,metallic,roughness,albedo,reflectance);

        // apply 'global illumination'
        vec3 ambient_result = uniforms.params.ambient_light_color.rgb * albedo;

        // combine all lighting
        vec3 result = key_result * key_shadow + (sky_result * sky_shadow + ambient_result) * occlusion;
        //vec3 result = key_result + (sky_result + ambient_result) * occlusion;

        // calculate fog
        float fog = clamp(16.0 * uniforms.params.background_color.a * ndist,0.0,1.0);

        // and mix with fog
        pixel = mix(result,pixel,fog);

        //pixel = vec3(0.5) + 0.5 * n;
    }

    return pixel;
}
