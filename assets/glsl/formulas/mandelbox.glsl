#define MANDELBOX_FOLD 1.0
#define MANDELBOX_SCALE 1.9
#define MANDELBOX_RADIUS 0.13

#define MANDELBOX_ROTATION MAT3(0.9985012,-0.0055561,0.0544474, -0.0000000,0.9948338,0.1015173, -0.0547301,-0.1013652,0.9933427)
//#define MANDELBOX_ROTATION MAT3(0.9027011,-0.3816559,0.1986693,0.4057410,0.9087359,-0.0978434,-0.1431954,0.1689316,0.9751703)
//#define MANDELBOX_ROTATION MAT3(0.9987503,0.0000000,0.0499792,0.0004998,0.9999500,-0.0099873,-0.0499767,0.0099998,0.9987003)

void mandelbox(inout vec3 v,inout float dr,vec3 c) {
    v = clamp(v, -1.0, 1.0) * 2.0 - v;
    float r2 = dot(v,v);
    if (r2 < MANDELBOX_RADIUS) {
        v *= 2.0;
        dr *= 2.0;
    }
    else if (r2 < 1.0) {
        float t = 1.0 / r2;
        v *= t;
        dr *= t;
    }
    //v = MANDELBOX_ROTATION * v;
    v = MANDELBOX_SCALE * v;
    v += c;
    dr = MANDELBOX_SCALE * dr + 1.0;
}
