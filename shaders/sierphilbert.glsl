#define SIERP_SCALE 3
#define SIERP_EDGE1 2
#define SIERP_EDGE2 1

void sierphilbert(inout VEC3 v,inout FLOAT dr,VEC3 c) {

    // v = rotation1 * v;

    v.x = -v.x;
    if (v.x >= v.y) { v = v.yxz; }
    if (v.x >= v.z) { v = v.zyx; }
    v.x = -v.x;
    v = v.yxz;
    v.x = -v.x;
    if (v.x >= v.z) { v = v.zyx; }
    v.x = -v.x;
    v = v.yxz;

    if (0 < SIERP_EDGE1) {
        v.y = SIERP_EDGE1 - abs(SIERP_EDGE1 - v.y);
    }
    if (0 < SIERP_EDGE2) {
        v.x = SIERP_EDGE2 - abs(SIERP_EDGE2 - v.x);
    }

    //v = rotation2 * v;

    v -= (SIERP_SCALE - 1); // * SIERP_CSCALE

    v += c;

    dr = dr * SIERP_SCALE + 1.0;
}
