/*
float bulb(vec3 p,vec3 center) {
    VEC3 c = p - center;
    VEC33 v = c;
    if(length(v) > 1.5) return float(length(v) - 1.2);
    FLOAT dr = 1.0;
    FLOAT r = length(v);
    for (int i = 0; i < MAX_ITERATION; i++) {
        r = length(v);
        if (r > ESCAPE_DISTANCE) break;
        FLOAT r2 = r * r;
        FLOAT r4 = r2 * r2;
        FLOAT r7 = r * r2 * r4;
        FLOAT r8 = r4 * r4;
        dr = 8.0 * r7 * dr + 1.0;
        FLOAT theta = 8.0 * acos(v.z / r);
        FLOAT phi = 8.0 * atan(v.y,v.x);
        FLOAT sinTheta = sin(theta);
        v = r8 * VEC3(sinTheta * cos(phi),sinTheta * sin(phi),cos(theta)) + c;
    }
    return 0.5 * log(float(r)) * float(r) / float(dr);
}
*/
