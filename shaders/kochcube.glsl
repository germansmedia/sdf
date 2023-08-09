//#define KOCH_POST_SCALE 0.148106
#define KOCH_POST_SCALE 1.0
//#define KOCH_STRETCH 0.089698
#define KOCH_STRETCH 1.0
//#define KOCH_FOLD 0.049021
#define KOCH_FOLD 1.0
//#define KOCH_ADD VEC3(0.0396334,-0.081354,-0.003129)
#define KOCH_ADD VEC3(0.0,0.0,0.0)
#define KOCH_ROTATION MAT3(1.0,0.0,0.0, 0.0,1.0,0.0, 0.0,0.0,1.0)

void kochcube(inout VEC3 v,inout FLOAT dr,VEC3 c) {

    v = 3 * abs(v);
    if (v.y <= v.x) {
        v = v.yxz;
    }
    if (v.z <= v.x) {
        v = v.zyx;
    }
    if (v.z <= v.y) {
        v = v.xzy;
    }
    v = KOCH_ROTATION * (v + KOCH_ADD);
    v.z = KOCH_FOLD - abs(KOCH_FOLD - v.z);
    FLOAT ta = KOCH_STRETCH - 3;
    FLOAT tb = KOCH_STRETCH + 3;
    FLOAT td = v.x - ta;
    FLOAT tc = v.x - tb;
    if (v.y <= td) {
        v.x = td;
        v.y -= ta;
    }
    else if (v.y > tc) {
        v.x = tc;
    }
    else {
        v.x = v.y;
        v.y = tc;
    }
    v = KOCH_POST_SCALE * vec3(v.x / KOCH_STRETCH,v.y / KOCH_STRETCH,v.z) + c;
    dr = 3 * KOCH_POST_SCALE * dr / KOCH_STRETCH;
}
