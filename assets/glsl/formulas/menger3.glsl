#define MENGER3_SCALE 3.0
#define MENGER3_CSCALE vec3(1.0,1.0,1.0)
#define MENGER3_ROTATION1 mat3(0.9027011,-0.3816559,0.1986693,0.4057410,0.9087359,-0.0978434,-0.1431954,0.1689316,0.9751703)
#define MENGER3_ROTATION2 mat3(0.9987503,0.0000000,0.0499792,0.0004998,0.9999500,-0.0099873,-0.0499767,0.0099998,0.9987003)
#define MENGER3_ROTATION3 mat3(0.9985012,-0.0055561,0.0544474, -0.0000000,0.9948338,0.1015173, -0.0547301,-0.1013652,0.9933427)

void menger3(inout vec3 v,inout float dr,vec3 c) {
    v = abs(v);
    //v = MENGER3_ROTATION1 * v;
    //v = MENGER3_ROTATION2 * v;
    //v = MENGER3_ROTATION3 * v;
    if (v.x < v.y) { v = v.yxz; }
    if (v.x < v.z) { v = v.zyx; }
    if (v.y < v.z) { v = v.xzy; }
    //v = MENGER3_ROTATION3 * v;
    //v = MENGER3_ROTATION2 * v;
    //v = MENGER3_ROTATION1 * v;
    v *= MENGER3_SCALE;
    vec3 cs = MENGER3_CSCALE * (MENGER3_SCALE - 1);
    v.x -= cs.x;
    v.y -= cs.y;
    if (v.z > 0.5 * cs.z) { v.z -= cs.z; }
    dr = dr * MENGER3_SCALE;
}
