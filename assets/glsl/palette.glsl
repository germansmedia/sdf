vec4 sample_palette(float f) {
    return uniforms.params.palette[uint(floor(f)) & 7];
}
