#define RECIP_LIMITER1 1.37
#define RECIP_LIMITER2 0.67
#define RECIP_MUL1 1.775
#define RECIP_MUL2 1.0

void reciprocalz3b(inout VEC3 v,inout FLOAT dr,VEC3 c,inout FLOAT m) {
    FLOAT f0 = 1.0 / RECIP_LIMITER1 + 1.0 / RECIP_LIMITER2;
    FLOAT f1 = 1.0 / (abs(RECIP_MUL1 * v.z) + RECIP_LIMITER1);
    FLOAT f2 = 1.0 / (abs(RECIP_MUL2 * v.z * v.z) + RECIP_LIMITER2);
    v.z = sign(v.z) * (f0 - f1 - f2);
}
