#include "base.glsl"

vec3 brdf(in vec3 n,in vec3 l,in vec3 v,in float metallic,in float roughness,in vec3 albedo,in float reflectance) {
    
    vec3 h = normalize(l + v);
    float nl = clamp(dot(n,l),0.0,1.0);
    float nv = clamp(dot(n,v),0.0,1.0);
    float nh = clamp(dot(n,h),0.0,1.0);
    float vh = clamp(dot(v,h),0.0,1.0);

    // calculate Fresnel reflectance f0
    vec3 f0 = vec3(0.16 * reflectance * reflectance);
    f0 = mix(f0,albedo,metallic);

    // reflectance (Schlick)
    vec3 ref = f0 + (vec3(1.0) - f0) * pow(1.0 - vh,5.0);

    // normal distribution (GGX)
    float a = roughness * roughness;
    float aa = a * a;
    float b = nh * nh * (aa - 1.0) + 1.0;
    float bb = b * b;
    float ndf = OOPI * aa / bb;

    // geometry (Smith, Schlick)
    float k = 0.5 * a;
    float omk = 1.0 - k;
    float g1l = nl / (nl * omk + k);
    float g1v = nv / (nv * omk + k);
    float geo = g1l * g1v;

    // assemble BRDF
    vec3 brdf_diff = OOPI * (1.0 - metallic) * albedo * (vec3(1.0) - ref);
    vec3 brdf_spec = (ref * ndf * geo) / (4 * max(nl * nv,0.001));

    return brdf_diff + brdf_spec;
}

/*
scratch:

F0 for non-metals (only white specular reflections):

Quartz   0.04559
Ice      0.01791
Water    0.02037
Alcohol  0.01996
Glass    0.04
Milk     0.02218
Ruby     0.07727
Crystal  0.11111
Diamond  0.17197
Skin     0.028

F0 for metals (colored specular reflections):

Silver     0.971519  0.959915  0.915324
Aluminium  0.913183  0.921494  0.924524
Gold       1         0.765557  0.336057
Copper     0.955008  0.637527  0.538163
Chromium   0.549585  0.556114  0.554256
Nickel     0.659777  0.608679  0.525649
Titanium   0.541931  0.496791  0.449419
Cobalt     0.662124  0.654864  0.633732
Platinum   0.672411  0.637331  0.585456



rendering(V) = emitted(V) + integrate_over_hemisphere(BRDF(V,L) * radiance(L) * cos(theta) * domega)
irradiance(L) = dE(L) = radiance(L) * cos(theta) * domega

BRDF(V,L) = rhod / PI + (reflectance(V,H) * normal_distribution(H) * geometry(L,V)) / (4 * dot(N,L) * dot(N,V))
BRDF_diffuse(V,L) = rhod / PI
BRDF_specular(V,L) = (reflectance(V,H) * normal_distribution(H) * geometry(L,V)) / (4 * dot(N,L) * dot(N,V))

Fresnel:

    reflectance(V,H) = 1/2 * (reflectance_parallel(V,H)^2 + reflectance_perpendicular(V,H)^2)
    reflectance_parallel(V,H) = (e2 * cos(theta1) - e1 * cos(theta2)) / ((e2 * cos(theta1) + e1 * cos(theta2)))
    reflectance_perpendicular(V,H) = (e2 * cos(theta2) - e1 * cos(theta1)) / ((e2 * cos(theta2) + e1 * cos(theta1)))
    transmittance(V,H) = 1 - reflectance(V,H)

non-metals: diffuse part is colored, specular part is not
metals: diffuse part is 0, specular part is colored

Cook-Torrance:

    c = dot(V,H)
    g = sqrt(e2^2 + c^2 - 1)
    reflectance(V,H) = 1/2 * ((g - c) / (g + c))^2 * (1 + (((c * (g + c) - 1)) / ((c * (g - c) + 1)))^2)

Schlick:

    reflectance(V,H) = F0 + (1 - F0) * (1 - dot(V,H))^5, where F0 = fresnel reflectance of the material (RGB)

Blinn:

    normal_distribution(H) = 1 / (PI * a^2) dot(N,H)^(2 / (a^2) - 2), where a = roughness^2

Beckmann:

    normal_distribution(H) = 1 / (PI * a^2 * dot(N,H)^4) exp((dot(n,h)^2 - 1) / (a^2 * dot(N,H)^2))

GGX:

    normal_distribution(H) = a^2 / (PI * (dot(N,H)^2 * (a^2 - 1) + 1)^2)

Cook-Torrance:

    geometry(L,V) = min(1,2 * dot(N,H) * dot(N,V) / dot(V,H),2 * dot(N,H) * dot(N,L) / dot(V,H))

Smith:

    geometry(L,V) = G1(L) * G1(V)

Beckmann:

    G1(V) = (3.535 * c + 2.181 * c^2) / (1 + 2.276 * c + 2.577 * c^2) if c < 1.6, or 1 if c >= 1.6, where c = dot(N,V) / (a * sqrt(1 - dot(N,V)^2))

GGX:

    G1(V) = 2 * dot(N,V) / (dot(N,V) + sqrt(a^2 + (1 - a^2) * dot(N,V)^2))

Schlick:

    G1(V) = dot(N,V) / (dot(N,V) * (1 - k) + k), where k = a / 2

Take Schlick, Smith and GGX:

    reflectance(V,H) = F0 + (1 - F0) * (1 - dot(V,h))^5, where F0 = fresnel reflectance (RGB)
    normal_distribution(H) = a^2 / (PI * (dot(N,H)^2 * (a^2 - 1) + 1)^2), where a = roughness^2
    geometry(L,V) = G1(L) * G1(V), where G1(V) = dot(N,V) / (dot(N,V) * (1 - k) + k), where k = a / 2
*/
