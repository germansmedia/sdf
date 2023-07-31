# `koch_cube`

```
[OPTIONS]
.Version = 5
.DEoption = 2
.DEscale = 1
.SIpower = 1
.RStop = 5
.Double Post-scale = 1
.Double XY Stretch = 1
.Double Z Fold = 1
.Double X Add = 0
.Double Y Add = 0
.Double Z Add = 0
.3SingleAngles Rotation = 0
[CONSTANTS]
Double = 2
Double = 3
Double = 4
[CODE]
558BEC81EC200000005356578B75088B
7E308BD8DD03D9E1DC4F08DD1B90DD02
D9E1DC4F08DD1A90DD01D9E1DC4F08DD
19DD4108DC4F08DC77E8DC4FF0DD5908
90D9D0DD03DD02DED9DFE080E4417708
DD03DD02DD1BDD1ADD03DD01DED9DFE0
80E4417708DD03DD01DD1BDD19DD01DD
02DED9DFE080E4417608DD01DD02DD19
DD1AD9D0DD01DC47C8DD02DC47D0DD03
DC47D8D9C0D84FACD9C2D84FA8DEC1D9
C3D84FA4DEC1D9C1D84FC4D9C3D84FC0
DEC1D9C4D84FBCDEC1D9CAD84FB8D9CB
D84FB4DEC3D9CBD84FB0DEC2DD1BDD1A
DD1990DD47E0DC21D9E1D9E0DC47E0DD
1990DD4708D9C0DD47E8DEC1DD5DE8DD
47E8DEE9DD5DE090DD03DC65E0DD5DF8
90DD03DC65E8DD5DF090DD45F8DD02DE
D9DFE080E4417712DD45F8DD1BDD02DC
65E0DD1AEB2690909090DD45F0DD02DE
D9DFE080E441760990DD45F0DD1BEB0C
90DD02DD45F0DD1ADD1BD9D0DD03DC4F
F0DC77E8DD1BDD02DC4FF0DC77E8DD1A
DD01DC4FF0DD198BC35F5E5B89EC5DC2
0800
[END]
Cubic Koch fractal. Inspired from a Wikipedia image ( http://en.wikipedia.org/wiki/File:Quadratic_Koch_3D_(type1_stage2).png )
Added options for scale, stretching and other goodies
TIP - With default settings you don't see the fractal correctly; decrease DEstop, and use miniter=10 or something to see a better
plot. Be careful with scale and stretching tweaks, they greatly affect the render!
If you use xy stretch, the fractal will be less uniform but with some singularities.

----------------
  ; basic cubic fold (from Menger3 formula)
  ; scale has been replaced by a postscale 
  ; to fix some branch cut troubles
  x = abs(x)*3
  y = abs(y)*3
  z = abs(z)*3
  if y>x
  t=x
  x=y
  y=t
  endif
  if z>x
  t=x
  x=z
  z=t
  endif
  if z>y
  t=y
  y=z
  z=t
  endif
  x = x+xa, y=y+ya, z=z+za
  ( ... rotation of x,y,z on specified angles ... )
  ; basic vars
  a=3-stretch, b=3+stretch, c=x-a, d=x-b
  ; the routine works only for scale = 3, else some discontinuities will appear
  ; Now, draw the cube WITHOUT making discontinuites in the spatial vars
  ; bring what falls "out of the small cube zone" in the interval (-1;1) (next iter will take care)
  ; z must be folded to get a homogen look
  z=zfix-abs(zfix-z)
  if (c<y)
   ; out of the cube ... normalize
   x=c
   y=y-a ; NOT a-y -> discontinue
  elseif (d>y)
   ; point falls just over the small cube
   x=d
  else
   ; vertical faces of small cube (swap)
   t=x
   x=y
   y=d
  endif
  x = x/stretch, y=y/stretch ;NON conformal stretching
  x = x*postscale, y = y*postscale, z = z*postscale
----------------
FORMULA AND IMPLEMENTATION; 100% by Luca G.N. 2011.
```

## Disassembly

`[EBX]` = x
`[EDX]` = y
`[ECX]` = z
`[ECX+08]` = w

`[EDI+10]` = 4
`[EDI+08]` = 3
`[EDI]` = 2
`[EDI-08]` = 0.5
`[EDI-10]` = ps = Post-scale
`[EDI-18]` = s = XY Stretch
`[EDI-20]` = f = Z Fold
`[EDI-28]` = ax = X Add
`[EDI-30]` = ay = Y Add
`[EDI-38]` = az = Z Add
`[EDI-3C]` = r0
`[EDI-40]` = r1
`[EDI-44]` = r2
`[EDI-48]` = r3
`[EDI-4C]` = r4
`[EDI-50]` = r5
`[EDI-54]` = r6
`[EDI-58]` = r7
`[EDI-5C]` = r8
`[EBP-20]` = a
`[EBP-18]` = b
`[EBP-10]` = c
`[EBP-08]` = d
```
00000000 55                              PUSH EBP
00000001 8BEC                            MOV EBP,ESP
00000003 81EC20000000                    SUB ESP,00000020

00000009 53                              PUSH EBX
0000000A 56                              PUSH ESI
0000000B 57                              PUSH EDI

0000000C 8B7508                          MOV ESI,DWORD PTR [EBP+08]
0000000F 8B7E30                          MOV EDI,DWORD PTR [ESI+30]

00000012 8BD8                            MOV EBX,EAX

00000014 DD03                            FLD x           // x;
00000016 D9E1                            FABS            // |x|;
00000018 DC4F08                          FMUL 3          // 3 |x|;
0000001B DD1B                            FSTP x          // x = 3 |x|;

0000001D 90                              NOP

0000001E DD02                            FLD y           // y;
00000020 D9E1                            FABS            // |y|;
00000022 DC4F08                          FMUL 3          // 3 |y|;
00000025 DD1A                            FSTP y          // y = 3 |y|;

00000027 90                              NOP

00000028 DD01                            FLD z           // z;
0000002A D9E1                            FABS            // |z|;
0000002C DC4F08                          FMUL 3          // 3 |z|;
0000002F DD19                            FSTP z          // z = 3 |z|;

00000031 DD4108                          FLD w           // w;
00000034 DC4F08                          FMUL 3          // 3 w;
00000037 DC77E8                          FDIV s          // 3 w / s;
0000003A DC4FF0                          FMUL ps         // 3 ps w / s;
0000003D DD5908                          FSTP w          // w = 3 ps w / s;

00000040 90                              NOP
00000041 D9D0                            FNOP

00000043 DD03                            FLD x           // x;
00000045 DD02                            FLD y           // y; x;
00000047 DED9                            FCOMPP          // y ? x
00000049 DFE0                            FNSTSW AX
0000004B 80E441                          AND AH,41
0000004E 7708                            JA 00000058

00000050 DD03                            FLD x           // x;
00000052 DD02                            FLD y           // y; x;
00000054 DD1B                            FSTP x          // t = y; x;
00000056 DD1A                            FSTP y          // y = x, x = t

00000058 DD03                            FLD x           // x;
0000005A DD01                            FLD z           // z; x;
0000005C DED9                            FCOMPP          // z ? x
0000005E DFE0                            FNSTSW AX
00000060 80E441                          AND AH,41
00000063 7708                            JA 0000006D

00000065 DD03                            FLD x           // x;
00000067 DD01                            FLD z           // z; x;
00000069 DD1B                            FSTP x          // t = z; x;
0000006B DD19                            FSTP z          // z = x, x = t

0000006D DD01                            FLD z           // z;
0000006F DD02                            FLD y           // y; z;
00000071 DED9                            FCOMPP          // y ? z
00000073 DFE0                            FNSTSW AX
00000075 80E441                          AND AH,41
00000078 7608                            JBE 00000082

0000007A DD01                            FLD z           // z;
0000007C DD02                            FLD y           // y; z;
0000007E DD19                            FSTP z          // t = z; y;
00000080 DD1A                            FSTP y          // z = y, y = t

00000082 D9D0                            FNOP

00000084 DD01                            FLD z           // z;
00000086 DC47C8                          FADD az         // z + az;
00000089 DD02                            FLD y           // y; z + az;
0000008B DC47D0                          FADD ay         // y + ay; z + az;
0000008E DD03                            FLD x           // x; y + ay; z + az;
00000090 DC47D8                          FADD ax         // x + ax; y + ay; z + az;
00000093 D9C0                            FLD ST(0)       // x + ax; x + ax; y + ay; z + az;
00000095 D84FAC                          FMUL r6         // r6 (x + ax); x + ax; y + ay; z + az;
00000098 D9C2                            FLD ST(2)       // y + ay; r6 (x + ax); x + ax; y + ay; z + az;
0000009A D84FA8                          FMUL r7         // r7 (y + ay); r6 (x + ax); x + ax; y + ay; z + az;
0000009D DEC1                            FADDP ST(1),ST  // r6 (x + ax) + r7 (y + ay); x + ax; y + ay; z + az;
0000009F D9C3                            FLD ST(3)       // z + az; r6 (x + ax) + r7 (y + ay); x + ax; y + ay; z + az;
000000A1 D84FA4                          FMUL r8         // r8 (z + az); r6 (x + ax) + r7 (y + ay); x + ax; y + ay; z + az;
000000A4 DEC1                            FADDP ST(1),ST  // r6 (x + ax) + r7 (y + ay) + r8 (z + az); x + ax; y + ay; z + az;

000000A6 D9C1                            FLD ST(1)       // x + ax; r6 (x + ax) + r7 (y + ay) + r8 (z + az); x + ax; y + ay; z + az;
000000A8 D84FC4                          FMUL r0         // r0 (x + ax); r6 (x + ax) + r7 (y + ay) + r8 (z + az); x + ax; y + ay; z + az;
000000AB D9C3                            FLD ST(3)       // y + ay; r0 (x + ax); r6 (x + ax) + r7 (y + ay) + r8 (z + az); x + ax; y + ay; z + az;
000000AD D84FC0                          FMUL r1         // r1 (y + ay); r0 (x + ax); r6 (x + ax) + r7 (y + ay) + r8 (z + az); x + ax; y + ay; z + az;
000000B0 DEC1                            FADDP ST(1),ST  // r0 (x + ax) + r1 (y + ay); r6 (x + ax) + r7 (y + ay) + r8 (z + az); x + ax; y + ay; z + az;
000000B2 D9C4                            FLD ST(4)       // z + az; r0 (x + ax) + r1 (y + ay); r6 (x + ax) + r7 (y + ay) + r8 (z + az); x + ax; y + ay; z + az;
000000B4 D84FBC                          FMUL r2         // r2 (z + az); r0 (x + ax) + r1 (y + ay); r6 (x + ax) + r7 (y + ay) + r8 (z + az); x + ax; y + ay; z + az;
000000B7 DEC1                            FADDP ST(1),ST  // r0 (x + ax) + r1 (y + ay) + r2 (z + az); r6 (x + ax) + r7 (y + ay) + r8 (z + az); x + ax; y + ay; z + az;
000000B9 D9CA                            FXCH ST(2)      // x + ax; r6 (x + ax) + r7 (y + ay) + r8 (z + az); r0 (x + ax) + r1 (y + ay) + r2 (z + az); y + ay; z + az;
000000BB D84FB8                          FMUL r3         // r3 (x + ax); r6 (x + ax) + r7 (y + ay) + r8 (z + az); r0 (x + ax) + r1 (y + ay) + r2 (z + az); y + ay; z + az;
000000BE D9CB                            FXCH ST(3)      // y + ay; r6 (x + ax) + r7 (y + ay) + r8 (z + az); r0 (x + ax) + r1 (y + ay) + r2 (z + az); r3 (x + ax); z + az;
000000C0 D84FB4                          FMUL r4         // r4 (y + ay); r6 (x + ax) + r7 (y + ay) + r8 (z + az); r0 (x + ax) + r1 (y + ay) + r2 (z + az); r3 (x + ax); z + az;
000000C3 DEC3                            FADDP ST(3),ST  // r6 (x + ax) + r7 (y + ay) + r8 (z + az); r0 (x + ax) + r1 (y + ay) + r2 (z + az); r3 (x + ax) + r4 (y + ay); z + az;
000000C5 D9CB                            FXCH ST(3)      // z + az; r0 (x + ax) + r1 (y + ay) + r2 (z + az); r3 (x + ax) + r4 (y + ay); r6 (x + ax) + r7 (y + ay) + r8 (z + az);
000000C7 D84FB0                          FMUL r5         // r5 (z + az); r0 (x + ax) + r1 (y + ay) + r2 (z + az); r3 (x + ax) + r4 (y + ay); r6 (x + ax) + r7 (y + ay) + r8 (z + az);
000000CA DEC2                            FADDP ST(2),ST  // r0 (x + ax) + r1 (y + ay) + r2 (z + az); r3 (x + ax) + r4 (y + ay) + r5 (z + az); r6 (x + ax) + r7 (y + ay) + r8 (z + az);
000000CC DD1B                            FSTP x          // x = r0 (x + ax) + r1 (y + ay) + r2 (z + az), r3 (x + ax) + r4 (y + ay) + r5 (z + az); r6 (x + ax) + r7 (y + ay) + r8 (z + az);
000000CE DD1A                            FSTP y          // y = r3 (x + ax) + r4 (y + ay) + r5 (z + az), r6 (x + ax) + r7 (y + ay) + r8 (z + az);
000000D0 DD19                            FSTP z          // z = r6 (x + ax) + r7 (y + ay) + r8 (z + az)

000000D2 90                              NOP

000000D3 DD47E0                          FLD f           // f;
000000D6 DC21                            FSUB z          // f - z;
000000D8 D9E1                            FABS            // |f - z|;
000000DA D9E0                            FCHS            // -|f - z|;
000000DC DC47E0                          FADD f          // f - |f - z|;
000000DF DD19                            FSTP z          // z = f - |f - z|

000000E1 90                              NOP

000000E2 DD4708                          FLD 3           // 3;
000000E5 D9C0                            FLD ST(0)       // 3; 3;
000000E7 DD47E8                          FLD s           // s; 3; 3;
000000EA DEC1                            FADDP ST(1),ST  // 3 + s; 3;
000000EC DD5DE8                          FSTP b          // b = 3 + s
000000EF DD47E8                          FLD s           // s; 3;
000000F2 DEE9                            FSUBP ST(1),ST  // 3 - s;
000000F4 DD5DE0                          FSTP a          // a = 3 - s

000000F7 90                              NOP

000000F8 DD03                            FLD x           // x;
000000FA DC65E0                          FSUB a          // x - a;
000000FD DD5DF8                          FSTP c          // c = x - a

00000100 90                              NOP

00000101 DD03                            FLD x           // x;
00000103 DC65E8                          FSUB b          // x - b;
00000106 DD5DF0                          FSTP d          // d = x - b

00000109 90                              NOP

0000010A DD45F8                          FLD c           // c;
0000010D DD02                            FLD y           // y; c;
0000010F DED9                            FCOMPP          // y ? c
00000111 DFE0                            FNSTSW AX
00000113 80E441                          AND AH,41
00000116 7712                            JA 0000012A

00000118 DD45F8                          FLD c           // c;
0000011B DD1B                            FSTP x          // x = c
0000011D DD02                            FLD y           // y;
0000011F DC65E0                          FSUB a          // y - a;
00000122 DD1A                            FSTP y          // y = y - a
00000124 EB26                            JMP 0000014C

00000126 90                              NOP
00000127 90                              NOP
00000128 90                              NOP
00000129 90                              NOP

0000012A DD45F0                          FLD d           // d;
0000012D DD02                            FLD y           // y; d;
0000012F DED9                            FCOMPP          // y ? d
00000131 DFE0                            FNSTSW AX
00000133 80E441                          AND AH,41
00000136 7609                            JBE 00000141

00000138 90                              NOP

00000139 DD45F0                          FLD d           // d;
0000013C DD1B                            FSTP x          // x = d
0000013E EB0C                            JMP 0000014C

00000140 90                              NOP

00000141 DD02                            FLD y           // y;
00000143 DD45F0                          FLD d           // d; y;
00000146 DD1A                            FSTP y          // t = d, y;
00000148 DD1B                            FSTP x          // x = y, y = t
0000014A D9D0                            FNOP

0000014C DD03                            FLD x           // x;
0000014E DC4FF0                          FMUL ps         // ps * x;
00000151 DC77E8                          FDIV s          // ps * x / s;
00000154 DD1B                            FSTP x          // x = ps * x / s

00000156 DD02                            FLD y           // y;
00000158 DC4FF0                          FMUL ps         // ps * y;
0000015B DC77E8                          FDIV s          // ps * y / s;
0000015E DD1A                            FSTP y          // y = ps * y / s;

00000160 DD01                            FLD z           // z;
00000162 DC4FF0                          FMUL ps         // ps * z;
00000165 DD19                            FSTP z          // z = ps * z;

00000167 8BC3                            MOV EAX,EBX

00000169 5F                              POP EDI
0000016A 5E                              POP ESI
0000016B 5B                              POP EBX

0000016C 89EC                            MOV ESP,EBP
0000016E 5D                              POP EBP
0000016F C20800                          RET 0008
```

## Code

```rust
let mut x = 3 * x.abs();
let mut y = 3 * y.abs();
let mut z = 3 * z.abs();
w = 3 * ps * w / s;
if y > x { let t = x; x = y; y = t; }
if z > x { let t = x; x = z; z = t; }
if z > y { let t = y; y = z; z = t; }
x += ax;
y += ay;
z += az;
let rx = r0 * x + r1 * y + r2 * z;
let ry = r3 * x + r4 * y + r5 * z;
let rz = r6 * x + r7 * y + r8 * z;
x = rx;
y = ry;
z = rz;
z = f - (f - z).abs();
let a = 3 - s;
let b = 3 + s;
let c = x - a;
let d = x - b;
if c < y {
    x = c;
    y = y - a;
}
else if d > y {
    x = d;
}
else {
    x = y;
    y = d;
}
x = ps * x / s;
y = ps * y / s;
z = ps * z;
```
