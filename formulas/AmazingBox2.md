# `AmazingBox2`

```
[OPTIONS]
.Version = 2
.DEscale = 0.2
.DEoption = 2
.RStop = 1024
.SIpower = 2
.Double Scale = 2
.Boxscale Min R = 0.5
.Double Fold = 1
.3SingleAngles Rotate = 0
.Double Inv xC = .5
.Double Inv yC = 0
.Double Inv zC = 0
.DRecipro Inv Radius = 1
.DoubleAngle FoldX, XY angle = 0
.DoubleAngle FoldX, XZ angle = 0
[CODE]
558BEC56538B750889C38B763083C4D0DD46D8DD03DC8E74FFFFFFDD01DC8E7C
FFFFFFDEE9DD1C24DD03DC8E7CFFFFFFDD01DC8E74FFFFFFDEC1DD5C24089090
DD0424DC4E84DD02DC4E8CDEE990DD0424DC4E8CDD02DC4E84DEC1DD5C241090
D9C0D8E2D9E1D8C1D9C9D8C2D9E1DEE1DC4E84DD442410DC4E8CDEE990DC8E74
FFFFFFDD44240890DC8E7CFFFFFFDEE990909090909090909090DD02D9C0D8E3
D9E1D8C1D9C9D8C3D9E1DEE1DD01D9C0D8E4D9E1D8C1D9C9D8C4D9E1DEE190DD
469CDCC1DDD890DD46A4DCC2DDD890DD46ACDCC3DDD8D9C0D8C8D9C2D8C8DEC1
D9C3D8C8DEC1DC4E9490DC56E0DFE0D0EC7307DDD8DD46E8EB16D9E8D8D1DFE0
D0EC7207DDD8DC7EF0EB05DED9DD46F0DD4108D8C9DD5908DCCBDCCADEC990DD
469CDC4EF0DCE9DDD890DD46A4DC4EF0DCEADDD890DD46ACDC4EF0DCEBDDD890
D9CAD9C0D84EBCD9C2D84EB8DEC1D9C3D84EB4DEC1D9C1D84ED4D9C3D84ED0DE
C1D9C4D84ECCDEC1D9CAD84EC8D9CBD84EC4DEC3D9CBD84EC0DEC2D9CA8B7508
DC4628DD19DC4620DD1ADC4618DD1BDDD883C43089D85B5E5DC20800
[END]
```

## Disassembly

s = Scale
mr = Min R
f = Fold
r0..r8 = Rotate X, Rotate Y, Rotate Z as rotation matrix
ix = Inv xC
iy = Inv yC
iz = Inv zC
ir = Inv Radius
xy = FoldX, XY angle
xz = FoldX, XZ angle

`[EBX]` = x
`[EDX]` = y
`[ECX]` = z
`[ECX+08]` = w
`[ESI-08]` = 0.5
`[ESI-10]` = s
`[ESI-18]` = s/(mr mr)
`[ESI-20]` = mr mr
`[ESI-28]` = f
`[ESI-2C]` = r0
`[ESI-30]` = r1
`[ESI-34]` = r2
`[ESI-38]` = r3
`[ESI-3C]` = r4
`[ESI-40]` = r5
`[ESI-44]` = r6
`[ESI-48]` = r7
`[ESI-4C]` = r8
`[ESI-54]` = ix
`[ESI-5C]` = iy
`[ESI-64]` = iz
`[ESI-6C]` = 1/ir
`[ESI-74]` = sin(xy)
`[ESI-7C]` = cos(xy)
`[ESI-84]` = sin(xz)
`[ESI-8C]` = cos(xz)

```
00000000 55                              PUSH EBP
00000001 8BEC                            MOV EBP,ESP

00000003 56                              PUSH ESI
00000004 53                              PUSH EBX

00000005 8B7508                          MOV ESI,DWORD PTR [EBP+08]
00000008 89C3                            MOV EBX,EAX
0000000A 8B7630                          MOV ESI,DWORD PTR [ESI+30]

0000000D 83C4D0                          ADD ESP,FFFFFFD0

00000010 DD46D8                          FLD f             // f;

00000013 DD03                            FLD x             // x; f;
00000015 DC8E74FFFFFF                    FMUL cos(xz)      // cos(xz) x; f;
0000001B DD01                            FLD z             // z; cos(xz) x; f;
0000001D DC8E7CFFFFFF                    FMUL sin(xz)      // sin(xz) z; cos(xz) x; f;
00000023 DEE9                            FSUBP ST(1),ST    // cos(xz) x - sin(xz) z; f;
00000025 DD1C24                          FSTP t0           // t0 = cos(xz) x - sin(xz) z, f;

00000028 DD03                            FLD x             // x; f;
0000002A DC8E7CFFFFFF                    FMUL sin(xz)      // sin(xz) x; f;
00000030 DD01                            FLD z             // z; sin(xz) x; f;
00000032 DC8E74FFFFFF                    FMUL cos(xz)      // cos(xz) z; sin(xz) x; f;
00000038 DEC1                            FADDP ST(1),ST    // sin(xz) x + cos(xz) z; f;
0000003A DD5C2408                        FSTP t1           // t1 = sin(xz) x + cos(xz) z, f;

0000003E 90                              NOP
0000003F 90                              NOP

00000040 DD0424                          FLD t0            // t0; f;
00000043 DC4E84                          FMUL cos(xy)      // cos(xy) t0; f;
00000046 DD02                            FLD y             // y; cos(xy) t0; f;
00000048 DC4E8C                          FMUL sin(xy)      // sin(xy) y; cos(xy) t0; f;
0000004B DEE9                            FSUBP ST(1),ST    // cos(xy) t0 - sin(xy) y; f;

0000004D 90                              NOP

0000004E DD0424                          FLD t0            // t0; cos(xy) t0 - sin(xy) y; f;
00000051 DC4E8C                          FMUL sin(xy)      // sin(xy) t0; cos(xy) t0 - sin(xy) y; f;
00000054 DD02                            FLD y             // y; sin(xy) t0; cos(xy) t0 - sin(xy) y; f;
00000056 DC4E84                          FMUL cos(xy)      // cos(xy) y; sin(xy) t0; cos(xy) t0 - sin(xy) y; f;
00000059 DEC1                            FADDP ST(1),ST    // sin(xy) t0 + cos(xy) y; cos(xy) t0 - sin(xy) y; f;
0000005B DD5C2410                        FSTP t2           // t2 = sin(xy) t0 + cos(xy) y, cos(xy) t0 - sin(xy) y; f;

0000005F 90                              NOP

00000060 D9C0                            FLD ST(0)         // cos(xy) t0 - sin(xy) y; cos(xy) t0 - sin(xy) y; f;
00000062 D8E2                            FSUB ST,ST(2)     // cos(xy) t0 - sin(xy) y - f; cos(xy) t0 - sin(xy) y; f;
00000064 D9E1                            FABS              // |cos(xy) t0 - sin(xy) y - f|; cos(xy) t0 - sin(xy) y; f;
00000066 D8C1                            FADD ST,ST(1)     // |cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y; cos(xy) t0 - sin(xy) y; f;
00000068 D9C9                            FXCH ST(1)        // cos(xy) t0 - sin(xy) y; |cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y; f;
0000006A D8C2                            FADD ST,ST(2)     // cos(xy) t0 - sin(xy) y + f; |cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y; f;
0000006C D9E1                            FABS              // |cos(xy) t0 - sin(xy) y + f|; |cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y; f;
0000006E DEE1                            FSUBRP ST(1),ST   // |cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|; f;
00000070 DC4E84                          FMUL cos(xy)      // cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|); f;
00000073 DD442410                        FLD t2            // t2; cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|); f;
00000077 DC4E8C                          FMUL sin(xy)      // sin(xy) t2; cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|); f;
0000007A DEE9                            FSUBP ST(1),ST    // cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2; f;

0000007C 90                              NOP

0000007D DC8E74FFFFFF                    FMUL cos(xz)      // cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2; f;
00000083 DD442408                        FLD t1            // t1; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2; f;

00000087 90                              NOP

00000088 DC8E7CFFFFFF                    FMUL sin(xz)      // sin(xz) t1; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2; f;
0000008E DEE9                            FSUBP ST(1),ST    // cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;

00000090 90                              NOP
00000091 90                              NOP
00000092 90                              NOP
00000093 90                              NOP
00000094 90                              NOP
00000095 90                              NOP
00000096 90                              NOP
00000097 90                              NOP
00000098 90                              NOP
00000099 90                              NOP

0000009A DD02                            FLD y             // y; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;
0000009C D9C0                            FLD ST(0)         // y; y; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;
0000009E D8E3                            FSUB ST,ST(3)     // y - f; y; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;
000000A0 D9E1                            FABS              // |y - f|; y; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;
000000A2 D8C1                            FADD ST,ST(1)     // |y - f| + y; y; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;
000000A4 D9C9                            FXCH ST(1)        // y; |y - f| + y; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;
000000A6 D8C3                            FADD ST,ST(3)     // y + f; |y - f| + y; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;
000000A8 D9E1                            FABS              // |y + f|; |y - f| + y; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;
000000AA DEE1                            FSUBRP ST(1),ST   // |y - f| + y - |y + f|; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;

000000AC DD01                            FLD z             // z; |y - f| + y - |y + f|; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;
000000AE D9C0                            FLD ST(0)         // z; z; |y - f| + y - |y + f|; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;
000000B0 D8E4                            FSUB ST,ST(4)     // z - f; z; |y - f| + y - |y + f|; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;
000000B2 D9E1                            FABS              // |z - f|; z; |y - f| + y - |y + f|; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;
000000B4 D8C1                            FADD ST,ST(1)     // |z - f| + z; z; |y - f| + y - |y + f|; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;
000000B6 D9C9                            FXCH ST(1)        // z; |z - f| + z; |y - f| + y - |y + f|; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;
000000B8 D8C4                            FADD ST,ST(4)     // z + f; |z - f| + z; |y - f| + y - |y + f|; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;
000000BA D9E1                            FABS              // |z + f|; |z - f| + z; |y - f| + y - |y + f|; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;
000000BC DEE1                            FSUBRP ST(1),ST   // |z - f| + z - |z + f|; |y - f| + y - |y + f|; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;

000000BE 90                              NOP

000000BF DD469C                          FLD iz            // iz; |z - f| + z - |z + f|; |y - f| + y - |y + f|; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;
000000C2 DCC1                            FADD ST(1),ST     // iz; |z - f| + z - |z + f| + iz; |y - f| + y - |y + f|; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;
000000C4 DDD8                            FSTP ST(0)        // |z - f| + z - |z + f| + iz; |y - f| + y - |y + f|; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;

000000C6 90                              NOP

000000C7 DD46A4                          FLD iy            // iy; |z - f| + z - |z + f| + iz; |y - f| + y - |y + f|; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;
000000CA DCC2                            FADD ST(2),ST     // iy; |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;
000000CC DDD8                            FSTP ST(0)        // |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;

000000CE 90                              NOP

000000CF DD46AC                          FLD ix            // ix; |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1; f;
000000D2 DCC3                            FADD ST(3),ST     // ix; |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
000000D4 DDD8                            FSTP ST(0)        // |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
000000D6 D9C0                            FLD ST(0)         // |z - f| + z - |z + f| + iz; |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
000000D8 D8C8                            FMUL ST,ST(0)     // sqr(|z - f| + z - |z + f| + iz); |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
000000DA D9C2                            FLD ST(2)         // |y - f| + y - |y + f| + iy; sqr(|z - f| + z - |z + f| + iz); |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
000000DC D8C8                            FMUL ST,ST(0)     // sqr(|y - f| + y - |y + f| + iy); sqr(|z - f| + z - |z + f| + iz); |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
000000DE DEC1                            FADDP ST(1),ST    // sqr(|y - f| + y - |y + f| + iy) + sqr(|z - f| + z - |z + f| + iz); |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
000000E0 D9C3                            FLD ST(3)         // cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; sqr(|y - f| + y - |y + f| + iy) + sqr(|z - f| + z - |z + f| + iz); |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
000000E2 D8C8                            FMUL ST,ST(0)     // sqr(cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix); sqr(|y - f| + y - |y + f| + iy) + sqr(|z - f| + z - |z + f| + iz); |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
000000E4 DEC1                            FADDP ST(1),ST    // sqr(cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix) + sqr(|y - f| + y - |y + f| + iy) + sqr(|z - f| + z - |z + f| + iz); |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
000000E6 DC4E94                          FMUL 1/ir         // (sqr(cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix) + sqr(|y - f| + y - |y + f| + iy) + sqr(|z - f| + z - |z + f| + iz)) / ir; |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;

000000E9 90                              NOP

000000EA DC56E0                          FCOM mr mr        // compare (sqr(cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix) + sqr(|y - f| + y - |y + f| + iy) + sqr(|z - f| + z - |z + f| + iz)) / ir to mr mr, (sqr(cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix) + sqr(|y - f| + y - |y + f| + iy) + sqr(|z - f| + z - |z + f| + iz)) / ir; |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
000000ED DFE0                            FNSTSW AX
000000EF D0EC                            SHR AH,1
000000F1 7307                            JAE 000000FA

000000F3 DDD8                            FSTP ST(0)        // |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
000000F5 DD46E8                          FLD s/(mr mr)     // s / (mr mr); |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
000000F8 EB16                            JMP 00000110

000000FA D9E8                            FLD1              // 1; (sqr(cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix) + sqr(|y - f| + y - |y + f| + iy) + sqr(|z - f| + z - |z + f| + iz)) / ir; |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
000000FC D8D1                            FCOM ST(1)        // compare 1 to (sqr(cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix) + sqr(|y - f| + y - |y + f| + iy) + sqr(|z - f| + z - |z + f| + iz)) / ir, 1; (sqr(cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix) + sqr(|y - f| + y - |y + f| + iy) + sqr(|z - f| + z - |z + f| + iz)) / ir; |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
000000FE DFE0                            FNSTSW AX
00000100 D0EC                            SHR AH,1
00000102 7207                            JB 0000010B

00000104 DDD8                            FSTP ST(0)        // (sqr(cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix) + sqr(|y - f| + y - |y + f| + iy) + sqr(|z - f| + z - |z + f| + iz)) / ir; |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
00000106 DC7EF0                          FDIVR s           // s ir / (sqr(cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix) + sqr(|y - f| + y - |y + f| + iy) + sqr(|z - f| + z - |z + f| + iz)); |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
00000109 EB05                            JMP 00000110

0000010B DED9                            FCOMPP            // compare 1 to (sqr(cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix) + sqr(|y - f| + y - |y + f| + iy) + sqr(|z - f| + z - |z + f| + iz)) / ir, |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
0000010D DD46F0                          FLD s             // s; |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;

from here on, q is one of:
    s
    s / (mr mr)
    s ir / (sqr(cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix) + sqr(|y - f| + y - |y + f| + iy) + sqr(|z - f| + z - |z + f| + iz))

00000110 DD4108                          FLD w             // w; q; |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
00000113 D8C9                            FMUL ST,ST(1)     // q w; q; |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
00000115 DD5908                          FSTP w            // w = q w, q; |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
00000118 DCCB                            FMUL ST(3),ST     // q; |z - f| + z - |z + f| + iz; |y - f| + y - |y + f| + iy; q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
0000011A DCCA                            FMUL ST(2),ST     // q; |z - f| + z - |z + f| + iz; q (|y - f| + y - |y + f| + iy); q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
0000011C DEC9                            FMULP ST(1),ST    // q (|z - f| + z - |z + f| + iz); q (|y - f| + y - |y + f| + iy); q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;

0000011E 90                              NOP

0000011F DD469C                          FLD iz            // iz; q (|z - f| + z - |z + f| + iz); q (|y - f| + y - |y + f| + iy); q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
00000122 DC4EF0                          FMUL s            // s iz; q (|z - f| + z - |z + f| + iz); q (|y - f| + y - |y + f| + iy); q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
00000125 DCE9                            FSUB ST(1),ST     // s iz; q (|z - f| + z - |z + f| + iz) - s iz; q (|y - f| + y - |y + f| + iy); q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
00000127 DDD8                            FSTP ST(0)        // q (|z - f| + z - |z + f| + iz) - s iz; q (|y - f| + y - |y + f| + iy); q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;

00000129 90                              NOP

0000012A DD46A4                          FLD iy            // iy; q (|z - f| + z - |z + f| + iz) - s iz; q (|y - f| + y - |y + f| + iy); q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
0000012D DC4EF0                          FMUL s            // s iy; q (|z - f| + z - |z + f| + iz) - s iz; q (|y - f| + y - |y + f| + iy); q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
00000130 DCEA                            FSUB ST(2),ST     // s iy; q (|z - f| + z - |z + f| + iz) - s iz; q (|y - f| + y - |y + f| + iy) - s iy; q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
00000132 DDD8                            FSTP ST(0)        // q (|z - f| + z - |z + f| + iz) - s iz; q (|y - f| + y - |y + f| + iy) - s iy; q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;

00000134 90                              NOP

00000135 DD46AC                          FLD ix            // ix; q (|z - f| + z - |z + f| + iz) - s iz; q (|y - f| + y - |y + f| + iy) - s iy; q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
00000138 DC4EF0                          FMUL s            // s ix; q (|z - f| + z - |z + f| + iz) - s iz; q (|y - f| + y - |y + f| + iy) - s iy; q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix; f;
0000013B DCEB                            FSUB ST(3),ST     // s ix; q (|z - f| + z - |z + f| + iz) - s iz; q (|y - f| + y - |y + f| + iy) - s iy; q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix; f;
0000013D DDD8                            FSTP ST(0)        // q (|z - f| + z - |z + f| + iz) - s iz; q (|y - f| + y - |y + f| + iy) - s iy; q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix; f;

0000013F 90                              NOP

00000140 D9CA                            FXCH ST(2)        // q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix; q (|y - f| + y - |y + f| + iy) - s iy; q (|z - f| + z - |z + f| + iz) - s iz; f;
00000142 D9C0                            FLD ST(0)         // q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix; q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix; q (|y - f| + y - |y + f| + iy) - s iy; q (|z - f| + z - |z + f| + iz) - s iz; f;
00000144 D84EBC                          FMUL r6           // r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix; q (|y - f| + y - |y + f| + iy) - s iy; q (|z - f| + z - |z + f| + iz) - s iz; f;
00000147 D9C2                            FLD ST(2)         // q (|y - f| + y - |y + f| + iy) - s iy; r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix; q (|y - f| + y - |y + f| + iy) - s iy; q (|z - f| + z - |z + f| + iz) - s iz; f;
00000149 D84EB8                          FMUL r7           // r7 (q (|y - f| + y - |y + f| + iy) - s iy); r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix; q (|y - f| + y - |y + f| + iy) - s iy; q (|z - f| + z - |z + f| + iz) - s iz; f;
0000014C DEC1                            FADDP ST(1),ST    // r7 (q (|y - f| + y - |y + f| + iy) - s iy) + r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix; q (|y - f| + y - |y + f| + iy) - s iy; q (|z - f| + z - |z + f| + iz) - s iz; f;
0000014E D9C3                            FLD ST(3)         // q (|z - f| + z - |z + f| + iz) - s iz; r7 (q (|y - f| + y - |y + f| + iy) - s iy) + r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix; q (|y - f| + y - |y + f| + iy) - s iy; q (|z - f| + z - |z + f| + iz) - s iz; f;
00000150 D84EB4                          FMUL r8           // r8 (q (|z - f| + z - |z + f| + iz) - s iz); r7 (q (|y - f| + y - |y + f| + iy) - s iy) + r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix; q (|y - f| + y - |y + f| + iy) - s iy; q (|z - f| + z - |z + f| + iz) - s iz; f;
00000153 DEC1                            FADDP ST(1),ST    // r8 (q (|z - f| + z - |z + f| + iz) - s iz) + r7 (q (|y - f| + y - |y + f| + iy) - s iy) + r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix; q (|y - f| + y - |y + f| + iy) - s iy; q (|z - f| + z - |z + f| + iz) - s iz; f;
00000155 D9C1                            FLD ST(1)         // q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix; r8 (q (|z - f| + z - |z + f| + iz) - s iz) + r7 (q (|y - f| + y - |y + f| + iy) - s iy) + r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix; q (|y - f| + y - |y + f| + iy) - s iy; q (|z - f| + z - |z + f| + iz) - s iz; f;
00000157 D84ED4                          FMUL r0           // r0 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r8 (q (|z - f| + z - |z + f| + iz) - s iz) + r7 (q (|y - f| + y - |y + f| + iy) - s iy) + r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix; q (|y - f| + y - |y + f| + iy) - s iy; q (|z - f| + z - |z + f| + iz) - s iz; f;
0000015A D9C3                            FLD ST(3)         // q (|y - f| + y - |y + f| + iy) - s iy; r0 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r8 (q (|z - f| + z - |z + f| + iz) - s iz) + r7 (q (|y - f| + y - |y + f| + iy) - s iy) + r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix; q (|y - f| + y - |y + f| + iy) - s iy; q (|z - f| + z - |z + f| + iz) - s iz; f;
0000015C D84ED0                          FMUL r1           // r1 (q (|y - f| + y - |y + f| + iy) - s iy); r0 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r8 (q (|z - f| + z - |z + f| + iz) - s iz) + r7 (q (|y - f| + y - |y + f| + iy) - s iy) + r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix; q (|y - f| + y - |y + f| + iy) - s iy; q (|z - f| + z - |z + f| + iz) - s iz; f;
0000015F DEC1                            FADDP ST(1),ST    // r1 (q (|y - f| + y - |y + f| + iy) - s iy) + r0 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r8 (q (|z - f| + z - |z + f| + iz) - s iz) + r7 (q (|y - f| + y - |y + f| + iy) - s iy) + r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix; q (|y - f| + y - |y + f| + iy) - s iy; q (|z - f| + z - |z + f| + iz) - s iz; f;
00000161 D9C4                            FLD ST(4)         // q (|z - f| + z - |z + f| + iz) - s iz; r1 (q (|y - f| + y - |y + f| + iy) - s iy) + r0 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r8 (q (|z - f| + z - |z + f| + iz) - s iz) + r7 (q (|y - f| + y - |y + f| + iy) - s iy) + r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix; q (|y - f| + y - |y + f| + iy) - s iy; q (|z - f| + z - |z + f| + iz) - s iz; f;
00000163 D84ECC                          FMUL r2           // r2 (q (|z - f| + z - |z + f| + iz) - s iz); r1 (q (|y - f| + y - |y + f| + iy) - s iy) + r0 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r8 (q (|z - f| + z - |z + f| + iz) - s iz) + r7 (q (|y - f| + y - |y + f| + iy) - s iy) + r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix; q (|y - f| + y - |y + f| + iy) - s iy; q (|z - f| + z - |z + f| + iz) - s iz; f;
00000166 DEC1                            FADDP ST(1),ST    // r2 (q (|z - f| + z - |z + f| + iz) - s iz) + r1 (q (|y - f| + y - |y + f| + iy) - s iy) + r0 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r8 (q (|z - f| + z - |z + f| + iz) - s iz) + r7 (q (|y - f| + y - |y + f| + iy) - s iy) + r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix; q (|y - f| + y - |y + f| + iy) - s iy; q (|z - f| + z - |z + f| + iz) - s iz; f;
00000168 D9CA                            FXCH ST(2)        // q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix; r8 (q (|z - f| + z - |z + f| + iz) - s iz) + r7 (q (|y - f| + y - |y + f| + iy) - s iy) + r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r2 (q (|z - f| + z - |z + f| + iz) - s iz) + r1 (q (|y - f| + y - |y + f| + iy) - s iy) + r0 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); q (|y - f| + y - |y + f| + iy) - s iy; q (|z - f| + z - |z + f| + iz) - s iz; f;
0000016A D84EC8                          FMUL r3           // r3 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r8 (q (|z - f| + z - |z + f| + iz) - s iz) + r7 (q (|y - f| + y - |y + f| + iy) - s iy) + r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r2 (q (|z - f| + z - |z + f| + iz) - s iz) + r1 (q (|y - f| + y - |y + f| + iy) - s iy) + r0 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); q (|y - f| + y - |y + f| + iy) - s iy; q (|z - f| + z - |z + f| + iz) - s iz; f;
0000016D D9CB                            FXCH ST(3)        // q (|y - f| + y - |y + f| + iy) - s iy; r8 (q (|z - f| + z - |z + f| + iz) - s iz) + r7 (q (|y - f| + y - |y + f| + iy) - s iy) + r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r2 (q (|z - f| + z - |z + f| + iz) - s iz) + r1 (q (|y - f| + y - |y + f| + iy) - s iy) + r0 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r3 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); q (|z - f| + z - |z + f| + iz) - s iz; f;
0000016F D84EC4                          FMUL r4           // r4 (q (|y - f| + y - |y + f| + iy) - s iy); r8 (q (|z - f| + z - |z + f| + iz) - s iz) + r7 (q (|y - f| + y - |y + f| + iy) - s iy) + r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r2 (q (|z - f| + z - |z + f| + iz) - s iz) + r1 (q (|y - f| + y - |y + f| + iy) - s iy) + r0 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r3 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); q (|z - f| + z - |z + f| + iz) - s iz; f;
00000172 DEC3                            FADDP ST(3),ST    // r8 (q (|z - f| + z - |z + f| + iz) - s iz) + r7 (q (|y - f| + y - |y + f| + iy) - s iy) + r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r2 (q (|z - f| + z - |z + f| + iz) - s iz) + r1 (q (|y - f| + y - |y + f| + iy) - s iy) + r0 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r4 (q (|y - f| + y - |y + f| + iy) - s iy) + r3 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); q (|z - f| + z - |z + f| + iz) - s iz; f;
00000174 D9CB                            FXCH ST(3)        // q (|z - f| + z - |z + f| + iz) - s iz; r2 (q (|z - f| + z - |z + f| + iz) - s iz) + r1 (q (|y - f| + y - |y + f| + iy) - s iy) + r0 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r4 (q (|y - f| + y - |y + f| + iy) - s iy) + r3 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r8 (q (|z - f| + z - |z + f| + iz) - s iz) + r7 (q (|y - f| + y - |y + f| + iy) - s iy) + r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); f;
00000176 D84EC0                          FMUL r5           // r5 (q (|z - f| + z - |z + f| + iz) - s iz); r2 (q (|z - f| + z - |z + f| + iz) - s iz) + r1 (q (|y - f| + y - |y + f| + iy) - s iy) + r0 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r4 (q (|y - f| + y - |y + f| + iy) - s iy) + r3 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r8 (q (|z - f| + z - |z + f| + iz) - s iz) + r7 (q (|y - f| + y - |y + f| + iy) - s iy) + r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); f;
00000179 DEC2                            FADDP ST(2),ST    // r2 (q (|z - f| + z - |z + f| + iz) - s iz) + r1 (q (|y - f| + y - |y + f| + iy) - s iy) + r0 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r5 (q (|z - f| + z - |z + f| + iz) - s iz) + r4 (q (|y - f| + y - |y + f| + iy) - s iy) + r3 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r8 (q (|z - f| + z - |z + f| + iz) - s iz) + r7 (q (|y - f| + y - |y + f| + iy) - s iy) + r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); f;
0000017B D9CA                            FXCH ST(2)        // r8 (q (|z - f| + z - |z + f| + iz) - s iz) + r7 (q (|y - f| + y - |y + f| + iy) - s iy) + r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r5 (q (|z - f| + z - |z + f| + iz) - s iz) + r4 (q (|y - f| + y - |y + f| + iy) - s iy) + r3 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r2 (q (|z - f| + z - |z + f| + iz) - s iz) + r1 (q (|y - f| + y - |y + f| + iy) - s iy) + r0 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); f;
0000017D 8B7508                          MOV ESI,DWORD PTR [EBP+08]
00000180 DC4628                          FADD cz           // r8 (q (|z - f| + z - |z + f| + iz) - s iz) + r7 (q (|y - f| + y - |y + f| + iy) - s iy) + r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix) + cz; r5 (q (|z - f| + z - |z + f| + iz) - s iz) + r4 (q (|y - f| + y - |y + f| + iy) - s iy) + r3 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r2 (q (|z - f| + z - |z + f| + iz) - s iz) + r1 (q (|y - f| + y - |y + f| + iy) - s iy) + r0 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); f;
00000183 DD19                            FSTP z            // z = r8 (q (|z - f| + z - |z + f| + iz) - s iz) + r7 (q (|y - f| + y - |y + f| + iy) - s iy) + r6 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix) + cz, r5 (q (|z - f| + z - |z + f| + iz) - s iz) + r4 (q (|y - f| + y - |y + f| + iy) - s iy) + r3 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); r2 (q (|z - f| + z - |z + f| + iz) - s iz) + r1 (q (|y - f| + y - |y + f| + iy) - s iy) + r0 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); f;
00000185 DC4620                          FADD cy           // r5 (q (|z - f| + z - |z + f| + iz) - s iz) + r4 (q (|y - f| + y - |y + f| + iy) - s iy) + r3 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix) + cy; r2 (q (|z - f| + z - |z + f| + iz) - s iz) + r1 (q (|y - f| + y - |y + f| + iy) - s iy) + r0 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); f;
00000188 DD1A                            FSTP y            // y = r5 (q (|z - f| + z - |z + f| + iz) - s iz) + r4 (q (|y - f| + y - |y + f| + iy) - s iy) + r3 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix) + cy, r2 (q (|z - f| + z - |z + f| + iz) - s iz) + r1 (q (|y - f| + y - |y + f| + iy) - s iy) + r0 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix); f;
0000018A DC4618                          FADD cx           // r2 (q (|z - f| + z - |z + f| + iz) - s iz) + r1 (q (|y - f| + y - |y + f| + iy) - s iy) + r0 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix) + cx; f;
0000018D DD1B                            FSTP x            // x = r2 (q (|z - f| + z - |z + f| + iz) - s iz) + r1 (q (|y - f| + y - |y + f| + iy) - s iy) + r0 (q cos(xz) cos(xy) (|cos(xy) t0 - sin(xy) y - f| + cos(xy) t0 - sin(xy) y - |cos(xy) t0 - sin(xy) y + f|) - sin(xy) t2 - sin(xz) t1 + ix - s ix) + cx, f;
0000018F DDD8                            FSTP ST(0)

00000191 83C430                          ADD ESP,00000030

00000194 89D8                            MOV EAX,EBX

00000196 5B                              POP EBX
00000197 5E                              POP ESI

00000198 5D                              POP EBP
00000199 C20800                          RET 0008
```

## Code

```rust
let t0 = cos(xz) * x - sin(xz) * z;
let t1 = sin(xz) * x + cos(xz) * z;
let t2 = sin(xy) * t0 + cos(xy) * y;
let t3 = cos(xy) * t0 + sin(xy) * y;
let ab3 = (t3 - f).abs() + t3 - (t3 + f).abs();
let t4 = cos(xy) * ab3 - sin(xy) * t2;
let t5 = cos(xz) * t4 - sin(xz) * t1;
let ab1 = (y - f).abs() + y - (y + f).abs();
let ab2 = (z - f).abs() + z - (z + f).abs();
let sx = t5 + ix;
let sy = ab1 + iy;
let sz = ab2 + iz;
let tx = sx * sx;
let ty = sy * sy;
let tz = sz * sz;
let t = (tx + ty + tz) / ir;
let q = if t ? (mr * mr) {
    s / (mr * mr)
}
else if t ? 1 {
    s / t
}
else {
    s
};
let qx = q * sx - s * ix;
let qy = q * sy - s * iy;
let qz = q * sz - s * iz;
x = r2 * qz + r1 * qy + r0 * qz + cx
y = r5 * qz + r4 * qy + r3 * qz + cy
z = r8 * qz + r7 * qy + r6 * qz + cz
w = q * w;
```
