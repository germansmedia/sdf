#define ROTATE4D_CYZ 1.0
#define ROTATE4D_SYZ 0.0
#define ROTATE4D_CXZ -1.0
#define ROTATE4D_SXZ 0.0
#define ROTATE4D_CXY -1.0
#define ROTATE4D_SXY 0.0
#define ROTATE4D_CXW 1.0
#define ROTATE4D_SXW 0.0
#define ROTATE4D_CYW 1.0
#define ROTATE4D_SYW 0.0
#define ROTATE4D_CZW 1.0
#define ROTATE4D_SZW 0.0

void rotate4d(inout VEC3 v,inout FLOAT dr,VEC3 c) {
    // for this particular test, effectively we're only mirroring Y and Z axes
    v.y = -v.y;
    v.z = -v.z;

    // w doesn't change, and neither does dr
}
