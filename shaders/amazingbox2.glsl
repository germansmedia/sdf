#define AMAZING_SCALE 1.05
#define AMAZING_MINR2 0.04
#define AMAZING_FOLD VEC3(1.525,1.525,1.525)
//#define AMAZING_I VEC3(0.6342625,0.6342625,0.6342625)
#define AMAZING_RADIUS 1.0

#define AMAZING_I VEC3(1.2,0.63,0.7)

/*
#define AMAZING_SCALE 2.0
#define AMAZING_MINR2 0.25
#define AMAZING_FOLD VEC3(1.0,1.0,1.0)
#define AMAZING_I VEC3(0.5,0.0,0.0)
#define AMAZING_RADIUS 1.0
*/

void amazingbox2(inout VEC3 v,inout FLOAT dr,VEC3 c,inout FLOAT m) {

    // TODO: rotation

    v = abs(v -AMAZING_FOLD) + v - abs(v + AMAZING_FOLD) + AMAZING_I;

    FLOAT r2 = dot(v,v);
    if (r2 < AMAZING_MINR2) {
        FLOAT t = 1.0 / AMAZING_MINR2;
        v *= t;
        dr *= t;
    }
    else if (r2 < 1.0) {
        FLOAT t = 1.0 / r2;
        v *= t;
        dr *= t;
    }

    v = AMAZING_SCALE * v - AMAZING_SCALE * AMAZING_I;

    v += c;

    dr = AMAZING_SCALE * dr + 1.0;
}
