#define MANDELBOX_FOLD 1.0
#define MANDELBOX_SCALE 2.0
#define MANDELBOX_RADIUS 0.5

void mandelbox(inout VEC3 v,inout FLOAT dr,VEC3 c) {
    v = 2.0 * clamp(v,-MANDELBOX_FOLD,MANDELBOX_FOLD) - v;
    FLOAT r2 = dot(v,v);
    if (r2 < MANDELBOX_RADIUS) {
        FLOAT t = 2.0;
        v *= t;
        dr *= t;
    }
    else if (r2 < 1.0) {
        FLOAT t = 1.0 / r2;
        v *= t;
        dr *= t;
    }
    v = MANDELBOX_SCALE * v + c;
    dr = dr * MANDELBOX_SCALE + 1.0;
}
