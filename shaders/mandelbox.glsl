#define MANDELBOX_FOLD 1.0
#define MANDELBOX_SCALE 2.0
#define MANDELBOX_RADIUS 0.5

void mandelbox(inout VEC3 v,inout FLOAT dr,VEC3 c) {

    v = clamp(v, -1.0, 1.0) * 2.0 - v;

    FLOAT r2 = dot(v,v);
    if (r2 < MANDELBOX_RADIUS) {
        v *= 2.0;
        dr *= 2.0;
    }
    else if (r2 < 1.0) {
        FLOAT t = 1.0 / r2;
        v *= t;
        dr *= t;
    }

    v = MANDELBOX_SCALE * v;

    v += c;

    dr = MANDELBOX_SCALE * dr + 1.0;
}
