#ifndef _VIEWER_GLSL_
#define _VIEWER_GLSL_

#include "base.glsl"

#define ANISOTROPY_SQUARE 0
#define ANISOTROPY_RECT2 1
#define ANISOTROPY_RECT4 2
#define ANISOTROPY_RECT8 3
#define ANISOTROPY_RECT16 4

#define PHASE_FULL16X16 0
#define PHASE_RIGHT8X16 1
#define PHASE_BOTTOM8X8 2
#define PHASE_RIGHT4X8  3
#define PHASE_BOTTOM4X4 4
#define PHASE_RIGHT2X4  5
#define PHASE_BOTTOM2X2 6
#define PHASE_RIGHT1X2  7
#define PHASE_BOTTOM1X1 8

struct Config {
    uint width,height;
    uint tbd0;
    uint tbd1;
};

struct Progress {
    uint phase;  // one of PHASE_* (only for VR viewer)
    uint x,y;  // pixel offset of this block/pixel
    uint tbd0;
};

layout (std140,push_constant) readonly uniform Push {
    uint eye;
    uint face;
    uint anisotropy;
    uint y_offset;
} push;

layout (std140,binding = 0) readonly uniform Uniforms {
    Config config;
    Progress progress;
    // formula params
    March march;
    Render render;
} uniforms;

void get_block_spec(out ivec2 b,out vec2 c) {
    switch (push.anisotropy) {
        case ANISOTROPY_SQUARE:
            b.x = int(gl_GlobalInvocationID.x * 16);
            b.y = int(push.y_offset + gl_GlobalInvocationID.y * 16);
            c.x = float(b.x + uniforms.progress.x) + 0.5;
            c.y = float(b.y + uniforms.progress.y) + 0.5;
            break;
        case ANISOTROPY_RECT2:
            b.x = int(gl_GlobalInvocationID.x * 32);
            b.y = int(push.y_offset + gl_GlobalInvocationID.y * 16);
            c.x = float(b.x + 2 * uniforms.progress.x) + 1.0;
            c.y = float(b.y + uniforms.progress.y) + 0.5;
            break;
        case ANISOTROPY_RECT4:
            b.x = int(gl_GlobalInvocationID.x * 64);
            b.y = int(push.y_offset + gl_GlobalInvocationID.y * 16);
            c.x = float(b.x + 4 * uniforms.progress.x) + 2.0;
            c.y = float(b.y + uniforms.progress.y) + 0.5;
            break;
        case ANISOTROPY_RECT8:
            b.x = int(gl_GlobalInvocationID.x * 128);
            b.y = int(push.y_offset + gl_GlobalInvocationID.y * 16);
            c.x = float(b.x + 8 * uniforms.progress.x) + 4.0;
            c.y = float(b.y + uniforms.progress.y) + 0.5;
            break;
        case ANISOTROPY_RECT16:
            b.x = int(gl_GlobalInvocationID.x * 256);
            b.y = int(push.y_offset + gl_GlobalInvocationID.y * 16);
            c.x = float(b.x + 16 * uniforms.progress.x) + 8.0;
            c.y = float(b.y + uniforms.progress.y) + 0.5;
            break;
    }
}

void draw(in writeonly image2D image,in ivec2 p,in ivec2 extent,vec4 value) {
    ivec2 i;
    for (i.y = 0; i.y < extent.y; i.y++) {
        for (i.x = 0; i.x < extent.x; i.x++) {
            imageStore(image,p + i,value);
        }
    }
}

void draw_block(
    in writeonly image2D image,
    in ivec2 b,
    in vec4 value
) {
    switch (uniforms.progress.phase) {

        case PHASE_FULL16X16:
            switch (push.anisotropy) {
                case ANISOTROPY_SQUARE: draw(image,b,ivec2(16,16),value); break;
                case ANISOTROPY_RECT2: draw(image,b,ivec2(32,16),value); break;
                case ANISOTROPY_RECT4: draw(image,b,ivec2(64,16),value); break;
                case ANISOTROPY_RECT8: draw(image,b,ivec2(128,16),value); break;
                case ANISOTROPY_RECT16: draw(image,b,ivec2(256,16),value); break;
            }
            break;
        
        case PHASE_RIGHT8X16:
            switch (push.anisotropy) {
                case ANISOTROPY_SQUARE: draw(image,ivec2(b.x + uniforms.progress.x,b.y),ivec2(8,16),value); break;
                case ANISOTROPY_RECT2: draw(image,ivec2(b.x + 2 * uniforms.progress.x,b.y),ivec2(16,16),value); break;
                case ANISOTROPY_RECT4: draw(image,ivec2(b.x + 4 * uniforms.progress.x,b.y),ivec2(32,16),value); break;
                case ANISOTROPY_RECT8: draw(image,ivec2(b.x + 8 * uniforms.progress.x,b.y),ivec2(64,16),value); break;
                case ANISOTROPY_RECT16: draw(image,ivec2(b.x + 16 * uniforms.progress.x,b.y),ivec2(128,16),value); break;
            }
            break;

        case PHASE_BOTTOM8X8:
            switch (push.anisotropy) {
                case ANISOTROPY_SQUARE: draw(image,ivec2(b.x + uniforms.progress.x,b.y + uniforms.progress.y),ivec2(8,8),value); break;
                case ANISOTROPY_RECT2: draw(image,ivec2(b.x + 2 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(16,8),value); break;
                case ANISOTROPY_RECT4: draw(image,ivec2(b.x + 4 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(32,8),value); break;
                case ANISOTROPY_RECT8: draw(image,ivec2(b.x + 8 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(64,8),value); break;
                case ANISOTROPY_RECT16: draw(image,ivec2(b.x + 16 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(128,8),value); break;
            }
            break;

        case PHASE_RIGHT4X8:
            switch (push.anisotropy) {
                case ANISOTROPY_SQUARE: draw(image,ivec2(b.x + uniforms.progress.x,b.y + uniforms.progress.y),ivec2(4,8),value); break;
                case ANISOTROPY_RECT2: draw(image,ivec2(b.x + 2 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(8,8),value); break;
                case ANISOTROPY_RECT4: draw(image,ivec2(b.x + 4 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(16,8),value); break;
                case ANISOTROPY_RECT8: draw(image,ivec2(b.x + 8 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(32,8),value); break;
                case ANISOTROPY_RECT16: draw(image,ivec2(b.x + 16 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(64,8),value); break;
            }
            break;

        case PHASE_BOTTOM4X4:
            switch (push.anisotropy) {
                case ANISOTROPY_SQUARE: draw(image,ivec2(b.x + uniforms.progress.x,b.y + uniforms.progress.y),ivec2(4,4),value); break;
                case ANISOTROPY_RECT2: draw(image,ivec2(b.x + 2 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(8,4),value); break;
                case ANISOTROPY_RECT4: draw(image,ivec2(b.x + 4 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(16,4),value); break;
                case ANISOTROPY_RECT8: draw(image,ivec2(b.x + 8 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(32,4),value); break;
                case ANISOTROPY_RECT16: draw(image,ivec2(b.x + 16 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(64,4),value); break;
            }
            break;

        case PHASE_RIGHT2X4:
            switch (push.anisotropy) {
                case ANISOTROPY_SQUARE: draw(image,ivec2(b.x + uniforms.progress.x,b.y + uniforms.progress.y),ivec2(2,4),value); break;
                case ANISOTROPY_RECT2: draw(image,ivec2(b.x + 2 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(4,4),value); break;
                case ANISOTROPY_RECT4: draw(image,ivec2(b.x + 4 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(8,4),value); break;
                case ANISOTROPY_RECT8: draw(image,ivec2(b.x + 8 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(16,4),value); break;
                case ANISOTROPY_RECT16: draw(image,ivec2(b.x + 16 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(32,4),value); break;
            }
            break;

        case PHASE_BOTTOM2X2:
            switch (push.anisotropy) {
                case ANISOTROPY_SQUARE: draw(image,ivec2(b.x + uniforms.progress.x,b.y + uniforms.progress.y),ivec2(2,2),value); break;
                case ANISOTROPY_RECT2: draw(image,ivec2(b.x + 2 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(4,2),value); break;
                case ANISOTROPY_RECT4: draw(image,ivec2(b.x + 4 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(8,2),value); break;
                case ANISOTROPY_RECT8: draw(image,ivec2(b.x + 8 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(16,2),value); break;
                case ANISOTROPY_RECT16: draw(image,ivec2(b.x + 16 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(32,2),value); break;
            }
            break;

        case PHASE_RIGHT1X2:
            switch (push.anisotropy) {
                case ANISOTROPY_SQUARE: draw(image,ivec2(b.x + uniforms.progress.x,b.y + uniforms.progress.y),ivec2(1,2),value); break;
                case ANISOTROPY_RECT2: draw(image,ivec2(b.x + 2 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(2,2),value); break;
                case ANISOTROPY_RECT4: draw(image,ivec2(b.x + 4 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(4,2),value); break;
                case ANISOTROPY_RECT8: draw(image,ivec2(b.x + 8 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(8,2),value); break;
                case ANISOTROPY_RECT16: draw(image,ivec2(b.x + 16 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(16,2),value); break;
            }
            break;

        case PHASE_BOTTOM1X1:
            switch (push.anisotropy) {
                case ANISOTROPY_SQUARE: draw(image,ivec2(b.x + uniforms.progress.x,b.y + uniforms.progress.y),ivec2(1,1),value); break;
                case ANISOTROPY_RECT2: draw(image,ivec2(b.x + 2 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(2,1),value); break;
                case ANISOTROPY_RECT4: draw(image,ivec2(b.x + 4 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(4,1),value); break;
                case ANISOTROPY_RECT8: draw(image,ivec2(b.x + 8 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(8,1),value); break;
                case ANISOTROPY_RECT16: draw(image,ivec2(b.x + 16 * uniforms.progress.x,b.y + uniforms.progress.y),ivec2(16,1),value); break;
            }
            break;
    }
}

#endif
