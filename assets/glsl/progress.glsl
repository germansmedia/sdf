// SDF - block progress drawing and equirect anisotropy
// by Desmond Germans, 2023

#ifndef _PROGRESS_GLSL_
#define _PROGRESS_GLSL_

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

struct Progress {
    uint phase;  // one of PHASE_*
    uint x,y;  // pixel offset of this block/pixel
};

void get_block_spec(in uint anisotropy,in uint y_offset,in ViewConfig view,in Progress progress,out ivec2 b,out vec2 c) {
    switch (anisotropy) {
        case ANISOTROPY_SQUARE:
            b.x = int(gl_GlobalInvocationID.x * 16);
            b.y = int(y_offset + gl_GlobalInvocationID.y * 16);
            c.x = (float(b.x + progress.x) + 0.5) / float(view.extent.x);
            c.y = (float(b.y + progress.y) + 0.5) / float(view.extent.y);
            break;
        case ANISOTROPY_RECT2:
            b.x = int(gl_GlobalInvocationID.x * 32);
            b.y = int(y_offset + gl_GlobalInvocationID.y * 16);
            c.x = (float(b.x + 2 * progress.x) + 1.0) / float(view.extent.x);
            c.y = (float(b.y + progress.y) + 0.5) / float(view.extent.y);
            break;
        case ANISOTROPY_RECT4:
            b.x = int(gl_GlobalInvocationID.x * 64);
            b.y = int(y_offset + gl_GlobalInvocationID.y * 16);
            c.x = (float(b.x + 4 * progress.x) + 2.0) / float(view.extent.x);
            c.y = (float(b.y + progress.y) + 0.5) / float(view.extent.y);
            break;
        case ANISOTROPY_RECT8:
            b.x = int(gl_GlobalInvocationID.x * 128);
            b.y = int(y_offset + gl_GlobalInvocationID.y * 16);
            c.x = (float(b.x + 8 * progress.x) + 4.0) / float(view.extent.x);
            c.y = (float(b.y + progress.y) + 0.5) / float(view.extent.y);
            break;
        case ANISOTROPY_RECT16:
            b.x = int(gl_GlobalInvocationID.x * 256);
            b.y = int(y_offset + gl_GlobalInvocationID.y * 16);
            c.x = (float(b.x + 16 * progress.x) + 8.0) / float(view.extent.x);
            c.y = (float(b.y + progress.y) + 0.5) / float(view.extent.y);
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
    in ViewConfig view,
    in uint anisotropy,
    in Progress progress,
    in writeonly image2D image,
    in ivec2 b,
    in vec4 value
) {

    switch (progress.phase) {

        case PHASE_FULL16X16:
            switch (anisotropy) {
                case ANISOTROPY_SQUARE: draw(image,b,ivec2(16,16),value); break;
                case ANISOTROPY_RECT2: draw(image,b,ivec2(32,16),value); break;
                case ANISOTROPY_RECT4: draw(image,b,ivec2(64,16),value); break;
                case ANISOTROPY_RECT8: draw(image,b,ivec2(128,16),value); break;
                case ANISOTROPY_RECT16: draw(image,b,ivec2(256,16),value); break;
            }
            break;
        
        case PHASE_RIGHT8X16:
            switch (anisotropy) {
                case ANISOTROPY_SQUARE: draw(image,ivec2(b.x + progress.x,b.y),ivec2(8,16),value); break;
                case ANISOTROPY_RECT2: draw(image,ivec2(b.x + 2 * progress.x,b.y),ivec2(16,16),value); break;
                case ANISOTROPY_RECT4: draw(image,ivec2(b.x + 4 * progress.x,b.y),ivec2(32,16),value); break;
                case ANISOTROPY_RECT8: draw(image,ivec2(b.x + 8 * progress.x,b.y),ivec2(64,16),value); break;
                case ANISOTROPY_RECT16: draw(image,ivec2(b.x + 16 * progress.x,b.y),ivec2(128,16),value); break;
            }
            break;

        case PHASE_BOTTOM8X8:
            switch (anisotropy) {
                case ANISOTROPY_SQUARE: draw(image,ivec2(b.x + progress.x,b.y + progress.y),ivec2(8,8),value); break;
                case ANISOTROPY_RECT2: draw(image,ivec2(b.x + 2 * progress.x,b.y + progress.y),ivec2(16,8),value); break;
                case ANISOTROPY_RECT4: draw(image,ivec2(b.x + 4 * progress.x,b.y + progress.y),ivec2(32,8),value); break;
                case ANISOTROPY_RECT8: draw(image,ivec2(b.x + 8 * progress.x,b.y + progress.y),ivec2(64,8),value); break;
                case ANISOTROPY_RECT16: draw(image,ivec2(b.x + 16 * progress.x,b.y + progress.y),ivec2(128,8),value); break;
            }
            break;

        case PHASE_RIGHT4X8:
            switch (anisotropy) {
                case ANISOTROPY_SQUARE: draw(image,ivec2(b.x + progress.x,b.y + progress.y),ivec2(4,8),value); break;
                case ANISOTROPY_RECT2: draw(image,ivec2(b.x + 2 * progress.x,b.y + progress.y),ivec2(8,8),value); break;
                case ANISOTROPY_RECT4: draw(image,ivec2(b.x + 4 * progress.x,b.y + progress.y),ivec2(16,8),value); break;
                case ANISOTROPY_RECT8: draw(image,ivec2(b.x + 8 * progress.x,b.y + progress.y),ivec2(32,8),value); break;
                case ANISOTROPY_RECT16: draw(image,ivec2(b.x + 16 * progress.x,b.y + progress.y),ivec2(64,8),value); break;
            }
            break;

        case PHASE_BOTTOM4X4:
            switch (anisotropy) {
                case ANISOTROPY_SQUARE: draw(image,ivec2(b.x + progress.x,b.y + progress.y),ivec2(4,4),value); break;
                case ANISOTROPY_RECT2: draw(image,ivec2(b.x + 2 * progress.x,b.y + progress.y),ivec2(8,4),value); break;
                case ANISOTROPY_RECT4: draw(image,ivec2(b.x + 4 * progress.x,b.y + progress.y),ivec2(16,4),value); break;
                case ANISOTROPY_RECT8: draw(image,ivec2(b.x + 8 * progress.x,b.y + progress.y),ivec2(32,4),value); break;
                case ANISOTROPY_RECT16: draw(image,ivec2(b.x + 16 * progress.x,b.y + progress.y),ivec2(64,4),value); break;
            }
            break;

        case PHASE_RIGHT2X4:
            switch (anisotropy) {
                case ANISOTROPY_SQUARE: draw(image,ivec2(b.x + progress.x,b.y + progress.y),ivec2(2,4),value); break;
                case ANISOTROPY_RECT2: draw(image,ivec2(b.x + 2 * progress.x,b.y + progress.y),ivec2(4,4),value); break;
                case ANISOTROPY_RECT4: draw(image,ivec2(b.x + 4 * progress.x,b.y + progress.y),ivec2(8,4),value); break;
                case ANISOTROPY_RECT8: draw(image,ivec2(b.x + 8 * progress.x,b.y + progress.y),ivec2(16,4),value); break;
                case ANISOTROPY_RECT16: draw(image,ivec2(b.x + 16 * progress.x,b.y + progress.y),ivec2(32,4),value); break;
            }
            break;

        case PHASE_BOTTOM2X2:
            switch (anisotropy) {
                case ANISOTROPY_SQUARE: draw(image,ivec2(b.x + progress.x,b.y + progress.y),ivec2(2,2),value); break;
                case ANISOTROPY_RECT2: draw(image,ivec2(b.x + 2 * progress.x,b.y + progress.y),ivec2(4,2),value); break;
                case ANISOTROPY_RECT4: draw(image,ivec2(b.x + 4 * progress.x,b.y + progress.y),ivec2(8,2),value); break;
                case ANISOTROPY_RECT8: draw(image,ivec2(b.x + 8 * progress.x,b.y + progress.y),ivec2(16,2),value); break;
                case ANISOTROPY_RECT16: draw(image,ivec2(b.x + 16 * progress.x,b.y + progress.y),ivec2(32,2),value); break;
            }
            break;

        case PHASE_RIGHT1X2:
            switch (anisotropy) {
                case ANISOTROPY_SQUARE: draw(image,ivec2(b.x + progress.x,b.y + progress.y),ivec2(1,2),value); break;
                case ANISOTROPY_RECT2: draw(image,ivec2(b.x + 2 * progress.x,b.y + progress.y),ivec2(2,2),value); break;
                case ANISOTROPY_RECT4: draw(image,ivec2(b.x + 4 * progress.x,b.y + progress.y),ivec2(4,2),value); break;
                case ANISOTROPY_RECT8: draw(image,ivec2(b.x + 8 * progress.x,b.y + progress.y),ivec2(8,2),value); break;
                case ANISOTROPY_RECT16: draw(image,ivec2(b.x + 16 * progress.x,b.y + progress.y),ivec2(16,2),value); break;
            }
            break;

        case PHASE_BOTTOM1X1:
            switch (anisotropy) {
                case ANISOTROPY_SQUARE: draw(image,ivec2(b.x + progress.x,b.y + progress.y),ivec2(1,1),value); break;
                case ANISOTROPY_RECT2: draw(image,ivec2(b.x + 2 * progress.x,b.y + progress.y),ivec2(2,1),value); break;
                case ANISOTROPY_RECT4: draw(image,ivec2(b.x + 4 * progress.x,b.y + progress.y),ivec2(4,1),value); break;
                case ANISOTROPY_RECT8: draw(image,ivec2(b.x + 8 * progress.x,b.y + progress.y),ivec2(8,1),value); break;
                case ANISOTROPY_RECT16: draw(image,ivec2(b.x + 16 * progress.x,b.y + progress.y),ivec2(16,1),value); break;
            }
            break;
    }
}

#endif
