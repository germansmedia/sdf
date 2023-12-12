vec4 sample_palette(float f) {
    float nf = 7.0 * f;
    float r = fract(nf);
    return uniforms.params.palette[uint(floor(nf)) & 7];
}
