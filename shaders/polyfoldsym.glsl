#define POLY_ORDER 64.0

void polyfoldsym(inout VEC3 v,inout FLOAT dr,VEC3 c) {
    FLOAT t = POLY_ORDER * atan(v.y,v.x);
    int it = int(t);
    FLOAT a = -FLOAT(it) * 3.1415927 / (180.0 * POLY_ORDER);
    FLOAT ca = cos(a);
    FLOAT sa = sin(a);
    v.y = ca * v.y + sa * v.x;
    if ((it & 1) != 0) { v.y = -v.y; }
    v.x = sa * v.y - ca * v.x;
}
