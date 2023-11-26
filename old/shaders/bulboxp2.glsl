#define BULBOX_FOLD 1.0
#define BULBOX_SCALE 2.0
#define BULBOX_INV VEC3(0.0,0.0,0.0)
#define BULBOX_UNSHARPENING 0.0
#define BULBOX_INNER_SCALE -0.5
#define BULBOX_INNER_R 0.6
#define BULBOX_INNER_ZMUL 1.0

void bulboxp2(inout VEC3 v,inout FLOAT dr,VEC3 c,inout FLOAT m) {
    v = abs(v + VEC3(BULBOX_FOLD)) - abs(v - VEC3(BULBOX_FOLD)) + v;
    FLOAT rr = dot(v,v);
    FLOAT r = sqrt(rr);
    v = BULBOX_SCALE * v - BULBOX_INV;
    dr *= BULBOX_SCALE;
    if (r > 1.0) {
        v += c;
        dr += 1.0;
    }
    else {
        FLOAT zmul = sqrt(v.x * v.x + v.y * v.y + abs(BULBOX_UNSHARPENING));
        rr = BULBOX_INNER_SCALE / (rr * rr);
        FLOAT t1 = -(v.z * v.z) / (zmul * zmul) + 1;
        VEC3 q = VEC3((v.x * v.y - v.y * v.y) + t1 * rr,-v.x * v.y * rr,-2.0 * v.z * BULBOX_INNER_ZMUL * zmul);
        if (r < BULBOX_INNER_R) {
            q.y *= t1;
            v = q + c;
            dr += 1.0;
        }
        else {
            FLOAT t4 = (r - BULBOX_INNER_R) / (1.0 - BULBOX_INNER_R);
            q.z *= rr;
            v = mix(q,v,t4);
            dr += 1.0;
        }
    }
    v += BULBOX_INV;
}
