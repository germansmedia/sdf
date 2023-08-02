#define STT 0.816496580927726
#define SOT 0.5773502691896258
#define SOH 0.7071067811865475
#define BENESI_SCALE 2.0
#define BENESI_OFFSET 2.0

void benesipine(inout VEC3 v,inout FLOAT dr,VEC3 c) {
    FLOAT tx = (v.x * STT - v.z * SOT) * SOH;
    FLOAT z = abs(v.x * SOT + v.z * STT);
    FLOAT x = abs(tx - v.y * SOH);
    FLOAT y = abs(tx + v.y * SOH);
    tx = x * SOH + y * SOH;
    y = -x * SOH + y * SOH;
    x = tx * STT + z * SOT;
    z = -tx * SOT + z * STT;
    x = BENESI_SCALE * x + BENESI_OFFSET;
    y = BENESI_SCALE * y;
    z = BENESI_SCALE * z;
    FLOAT xt = x * x;
    FLOAT yt = y * y;
    FLOAT zt = z * z;
    FLOAT t = 2 * x / sqrt(yt + zt);
    v = VEC3(
        xt - yt - zt,
        2 * t * y * t * (yt - zt),
        t * (yt - zt)
    ) + c;
    FLOAT r = length(v);
    dr = 2.0 * dr * r;
}
