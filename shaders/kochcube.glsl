/*
#define KOCH_POST_SCALE 1.0
#define KOCH_STRETCH 1.0
#define KOCH_FOLD 1.0
#define KOCH_ADD VEC3(0.0,0.0,0.0)
#define KOCH_ROTATION MAT3(1.0,0.0,0.0, 0.0,1.0,0.0, 0.0,0.0,1.0)
*/

#define KOCH_POST_SCALE 0.148106
#define KOCH_STRETCH 0.089698
#define KOCH_FOLD 0.049021
#define KOCH_ADD VEC3(0.0396334,-0.081354,-0.003129)
//#define KOCH_ROTATION MAT3(0.9985012,-0.0000000,0.0547301, -0.0055561,0.9948338,0.1013652, -0.0544474,-0.1015173,0.9933427)
#define KOCH_ROTATION MAT3(0.9985012,-0.0055561,0.0544474, -0.0000000,0.9948338,0.1015173, -0.0547301,-0.1013652,0.9933427)

void kochcube(inout VEC3 v,inout FLOAT dr,VEC3 c) {

    v.x = 3.0 * abs(v.x);
    v.y = 3.0 * abs(v.y);
    v.z = 3.0 * abs(v.z);

    if (v.x < v.y) { v = v.yxz; }
    if (v.x < v.z) { v = v.zyx; }
    if (v.y < v.z) { v = v.xzy; }

    v += KOCH_ADD;
    v = KOCH_ROTATION * v;

    FLOAT ta = 3.0 - KOCH_STRETCH;
    FLOAT tb = 3.0 + KOCH_STRETCH;
    FLOAT tc = v.x - ta;
    FLOAT td = v.x - tb;
    v.z = KOCH_FOLD - abs(KOCH_FOLD - v.z);
    if (v.y > tc) {
        v.x = tc;
        v.y -= ta;
    }
    else if (v.y < td) {
        v.x = td;
    }
    else {
        v.x = v.y;
        v.y = td;
    }

    v.x /= KOCH_STRETCH;
    v.y /= KOCH_STRETCH;

    v = KOCH_POST_SCALE * v;

    dr = dr * 3 * KOCH_POST_SCALE / KOCH_STRETCH;
}
