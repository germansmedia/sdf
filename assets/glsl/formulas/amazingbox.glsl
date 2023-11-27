#define AMAZING_SCALE 2.2
#define AMAZING_MINR2 0.3
#define AMAZING_FOLD vec3(0.9,0.5,1.1)
#define AMAZING_I vec3(0.5,0.0,0.0)
#define AMAZING_RADIUS 1.0

void amazingbox(inout vec3 v,inout float dr,vec3 c) {

    v = abs(v -AMAZING_FOLD) + v - abs(v + AMAZING_FOLD) + AMAZING_I;
    float r2 = dot(v,v);
    if (r2 < AMAZING_MINR2) {
        float t = 1.0 / AMAZING_MINR2;
        v *= t;
        dr *= t;
    }
    else if (r2 < 1.0) {
        float t = 1.0 / r2;
        v *= t;
        dr *= t;
    }
    v = AMAZING_SCALE * v - AMAZING_SCALE * AMAZING_I;
    v += c;
    dr = AMAZING_SCALE * dr + 1.0;
}
