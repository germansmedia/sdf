#define AMAZING_SCALE 1.05
#define AMAZING_MINR 0.2
#define AMAZING_FOLD 1.525
#define AMAZING_IX 0.6342625
#define AMAZING_IY 0
#define AMAZING_IZ 0
#define AMAZING_RADIUS 1

void amazingbox2(inout VEC3 v,inout FLOAT dr,VEC3 c) {
    VEC3 f = VEC3(AMAZING_FOLD,AMAZING_FOLD,AMAZING_FOLD);
    VEC3 ab = abs(v - f) + v - abs(v + f);
    VEC3 i = VEC3(AMAZING_IX,AMAZING_IY,AMAZING_IZ);
    VEC3 s = ab + i;
    FLOAT t = dot(s,s);
    FLOAT q = AMAZING_SCALE;
    if (t < AMAZING_MINR * AMAZING_MINR) {
        q = AMAZING_SCALE / (AMAZING_MINR * AMAZING_MINR);
    }
    else if (t < 1.0) {
        q = AMAZING_SCALE / t;
    }
    v = q * s - AMAZING_SCALE * i;
    dr = q * dr;
}
