vec3 rotate_x(vec3 p,float a) {
    float s = sin(a);
    float c = cos(a);
    return vec3(p.x,c * p.y + s * p.z,-s * p.y + c * p.z);
}

vec3 rotate_y(vec3 p,float a) {
    float s = sin(a);
    float c = cos(a);
    return vec3(c * p.x + s * p.z,p.y,-s * p.x + c * p.z);
}

vec3 rotate_z(vec3 p,float a) {
    float s = sin(a);
    float c = cos(a);
    return vec3(c * p.x + s * p.y,-s * p.x + c * p.y,p.z);
}
