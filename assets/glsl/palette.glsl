vec4 sample_palette(float f) {
    float nf = 4.0 * f;
    float r = fract(nf);
    switch(uint(floor(nf)) & 3) {
        case 0: return mix(uniforms.render.palette[0],uniforms.render.palette[1],r);
        case 1: return mix(uniforms.render.palette[1],uniforms.render.palette[2],r);
        case 2: return mix(uniforms.render.palette[2],uniforms.render.palette[3],r);
        case 3: return mix(uniforms.render.palette[3],uniforms.render.palette[0],r);
    }
}
