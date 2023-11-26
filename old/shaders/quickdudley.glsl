void quick_dudley(inout VEC3 v,inout FLOAT dr,VEC3 c,inout FLOAT m) {
    v = VEC3(
        v.x * v.x - 2.0 * v.z * v.y,
        v.z * v.z + 2.0 * v.y * v.x,
        v.y * v.y + 2.0 * v.x * v.z
    ) + c;
    dr = 2.0 * dr * length(v);
}
