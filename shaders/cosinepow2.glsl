#define Z_MULTIPLIER 1.0

void cosine_pow2(inout VEC3 v,inout VEC3 dr,VEC3 c) {
    FLOAT q = 2 * v.z / sqrt(v.x * v.x + v.y * v.y);
    v = VEC3(
        (v.y * v.y - v.x * v.x) * q,
        2 * v.x * v.y * q,
        (v.z * v.z - v.x * v.x - v.y * v.y) * Z_MULTIPLIER
    ) + c;
    dr = 2.0 * dr * length(v);
}
