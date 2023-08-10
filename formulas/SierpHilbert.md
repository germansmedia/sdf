`SierpHilbert`

```
[OPTIONS]
.Version = 2
.DEscale = 0.2
.DEoption = 2
.SIpower = 2
.RStop = 1024
.Double Scale = 3
.Double CScale X = 1
.Double CScale Y = 1
.Double CScale Z = 1
.3SingleAngles Rotation1 = 0
.3SingleAngles Rotation2 = 0
.Double Edge1 = 2
.Double Edge2 = 1
[CODE]
558BEC535689C38B75088B7630DD01DD02DD03D9C0D84EBCD9C2D84EB8DEC1D9
C3D84EB4DEC1D9C1D84ED4D9C3D84ED0DEC1D9C4D84ECCDEC1D9CAD84EC8D9CB
D84EC4DEC3D9CBD84EC0DEC2D9E0D8D1DFE0D0EC7202D9C9D8D2DFE0D0EC7202
D9CAD9E0D9C9D9E0D8D2DFE0D0EC7202D9CAD9E0D9C99090DD4688D9EED8D9DF
E0D0EC730BD8E2D9E1D9E0DC4688D9CADDD89090DD4680D9EED8D9DFE0D0EC73
0BD8E1D9E1D9E0DC4680D9C9DDD89090D9D0D9C0D84E98D9C2D84E94DEC1D9C3
D84E90DEC1D9C1D84EB0D9C3D84EACDEC1D9C4D84EA8DEC1D9CAD84EA4D9CBD8
4EA0DEC3D9CBD84E9CDEC2DD46F0DD4108D8C9DD5908DCCBDCCADCC9D9E8DEE9
D9C0DC4ED8DEECD9C0DC4EE0DEEBDC4EE8DEE9DD1BDD1ADD1989D85E5B5DC208
00
[END]

This fractal is an hybridation of Sierpinski and Hilbert axioms, with these parameters.

In reality, an uncountable number of curves and fractals can be obtained varying the parameters.
```

## Disassembly

[ebx] = x
[edx] = y
[ecx] = z
[ecx+0x08] = w
[esi-0x08] = 0.5
[esi-0x10] = scale
[esi-0x18] = cscale.x
[esi-0x20] = cscale.y
[esi-0x28] = cscale.z
[esi-0x2c] = rotation1.x.x
[esi-0x30] = rotation1.x.y
[esi-0x34] = rotation1.x.z
[esi-0x38] = rotation1.y.x
[esi-0x3c] = rotation1.y.y
[esi-0x40] = rotation1.y.z
[esi-0x44] = rotation1.z.x
[esi-0x48] = rotation1.z.y
[esi-0x4c] = rotation1.z.z
[esi-0x50] = rotation2.x.x
[esi-0x54] = rotation2.x.y
[esi-0x58] = rotation2.x.z
[esi-0x5c] = rotation2.y.x
[esi-0x60] = rotation2.y.y
[esi-0x64] = rotation2.y.z
[esi-0x68] = rotation2.z.x
[esi-0x6c] = rotation2.z.y
[esi-0x70] = rotation2.z.z
[esi-0x78] = edge1
[esi-0x80] = edge2


```
0:  55                      push   ebp
1:  8b ec                   mov    ebp,esp

3:  53                      push   ebx
4:  56                      push   esi

5:  89 c3                   mov    ebx,eax

7:  8b 75 08                mov    esi,DWORD PTR [ebp+0x8]
a:  8b 76 30                mov    esi,DWORD PTR [esi+0x30]

// v = rotation1 * v

d:  dd 01                   fld    v.z              // v.z;
f:  dd 02                   fld    v.y              // v.y; v.z;
11: dd 03                   fld    v.x              // v.x; v.y; v.z;
13: d9 c0                   fld    st(0)            // v.x; v.x; v.y; v.z;
15: d8 4e bc                fmul   rotation1.z.x    // rotation1.z.x * v.x; v.x; v.y; v.z;
18: d9 c2                   fld    st(2)            // v.y; rotation1.z.x * v.x; v.x; v.y; v.z;
1a: d8 4e b8                fmul   rotation1.z.y    // rotation1.z.y * v.y; rotation1.z.x * v.x; v.x; v.y; v.z;
1d: de c1                   faddp  st(1),st         // rotation1.z.x * v.x + rotation1.z.y * v.y; v.x; v.y; v.z;
1f: d9 c3                   fld    st(3)            // v.z; rotation1.z.x * v.x + rotation1.z.y * v.y; v.x; v.y; v.z;
21: d8 4e b4                fmul   rotation1.z.z    // rotation1.z.z * v.z; rotation1.z.x * v.x + rotation1.z.y * v.y; v.x; v.y; v.z;
24: de c1                   faddp  st(1),st         // dot(rotation1.z,v); v.x; v.y; v.z;
26: d9 c1                   fld    st(1)            // v.x; dot(rotation1.z,v); v.x; v.y; v.z;
28: d8 4e d4                fmul   rotation1.x.x    // rotation1.x.x * v.x; dot(rotation1.z,v); v.x; v.y; v.z;
2b: d9 c3                   fld    st(3)            // v.y; rotation1.x.x * v.x; dot(rotation1.z,v); v.x; v.y; v.z;
2d: d8 4e d0                fmul   rotation1.x.y    // rotation1.x.y * v.y; rotation1.x.x * v.x; dot(rotation1.z,v); v.x; v.y; v.z;
30: de c1                   faddp  st(1),st         // rotation1.x.x * v.x + rotation1.x.y * v.y; dot(rotation1.z,v); v.x; v.y; v.z;
32: d9 c4                   fld    st(4)            // v.z; rotation1.x.x * v.x + rotation1.x.y * v.y; dot(rotation1.z,v); v.x; v.y; v.z;
34: d8 4e cc                fmul   rotation1.x.z    // rotation1.x.z * v.z; rotation1.x.x * v.x + rotation1.x.y * v.y; dot(rotation1.z,v); v.x; v.y; v.z;
37: de c1                   faddp  st(1),st         // dot(rotation1.x,v); dot(rotation1.z,v); v.x; v.y; v.z;
39: d9 ca                   fxch   st(2)            // v.x; dot(rotation1.z,v); dot(rotation1.x,v); v.y; v.z;
3b: d8 4e c8                fmul   rotation1.y.x    // rotation1.y.x * v.x; dot(rotation1.z,v); dot(rotation1.x,v); v.y; v.z;
3e: d9 cb                   fxch   st(3)            // v.y; dot(rotation1.z,v); dot(rotation1.x,v); rotation1.y.x * v.x; v.z;
40: d8 4e c4                fmul   rotation1.y.y    // rotation1.y.y * v.y; dot(rotation1.z,v); dot(rotation1.x,v); rotation1.y.x * v.x; v.z;
43: de c3                   faddp  st(3),st         // dot(rotation1.z,v); dot(rotation1.x,v); rotation1.y.x * v.x + rotation1.y.y * v.y; v.z;
45: d9 cb                   fxch   st(3)            // v.z; dot(rotation1.x,v); rotation1.y.x * v.x + rotation1.y.y * v.y; dot(rotation1.z,v);
47: d8 4e c0                fmul   rotation1.y.z    // rotation1.y.z * v.z; dot(rotation1.x,v); rotation1.y.x * v.x + rotation1.y.y * v.y; dot(rotation1.z,v);
4a: de c2                   faddp  st(2),st         // dot(rotation1.x,v); dot(rotation1.y,v); dot(rotation1.z,v);

v.x = -v.x;

4c: d9 e0                   fchs                    // x = -dot(rotation1.x,v); y = dot(rotation1.y,v); z = dot(rotation1.z,v);

if (v.x >= v.y) swap v.x and v.y

4e: d8 d1                   fcom   st(1)            // x ? y, x; y; z;
50: df e0                   fnstsw ax               // stsw = B, C3, TOP2, TOP1, TOP0, C2, C1, C0, IR, SF, P, U, O, Z, D, I
52: d0 ec                   shr    ah,1             // CF = C0, OF = C0, SF = 0, ZF = ah == 0, PF = ah bits even
54: 72 02                   jb     0x58             // jump if C0, jump if ST(0) < Source, jump if x < y

56: d9 c9                   fxch   st(1)            // swap x and y

if (v.x >= v.z) swap v.x and v.z

58: d8 d2                   fcom   st(2)            // x ? z, x; y; z;
5a: df e0                   fnstsw ax
5c: d0 ec                   shr    ah,1
5e: 72 02                   jb     0x62             // jump if x < z

60: d9 ca                   fxch   st(2)            // swap x and z

v.x = -v.x;
v = v.yxz;
v.y = -v.y;

62: d9 e0                   fchs                    // -x; y; z;
64: d9 c9                   fxch   st(1)            // y; -x; z;
66: d9 e0                   fchs                    // -y; -x; z;

if (v.x >= v.z) swap v.x and v.z

68: d8 d2                   fcom   st(2)            // -y ? z, -y; -x; z;
6a: df e0                   fnstsw ax
6c: d0 ec                   shr    ah,1
6e: 72 02                   jb     0x72             // jump if -y < z

70: d9 ca                   fxch   st(2)            // z; -x; -y;

v.x = -v.x;
v = v.yxz;

72: d9 e0                   fchs                    // y; -x; z;
74: d9 c9                   fxch   st(1)            // -x; y; z;

76: 90                      nop
77: 90                      nop

if (0 < edge1) y = edge1 - |edge1 - y|

78: dd 46 88                fld    edge1            // edge1; x; y; z;
7b: d9 ee                   fldz                    // 0; edge1; x; y; z;
7d: d8 d9                   fcomp  st(1)            // 0 ? edge1, edge1; x; y; z;
7f: df e0                   fnstsw ax
81: d0 ec                   shr    ah,1
83: 73 0b                   jae    0x90             // jump if !C0, jump if ST(0) >= Source, jump if 0 >= edge1

85: d8 e2                   fsub   st,st(2)         // edge1 - y; x; y; z;
87: d9 e1                   fabs                    // |edge1 - y|; x; y; z;
89: d9 e0                   fchs                    // -|edge1 - y|; x; y; z;
8b: dc 46 88                fadd   edge1            // edge1 - |edge1 - y|; x; y; z;
8e: d9 ca                   fxch   st(2)            // y; x; y2 = edge1 - |edge1 - t.y|; z;

90: dd d8                   fstp   st(0)            // x; y2; z;

92: 90                      nop
93: 90                      nop

if (0 < edge2) x = edge2 - |edge2 - x|

94: dd 46 80                fld    edge2            // edge2; x; y2; z;
97: d9 ee                   fldz                    // 0; edge2; x; y2; z;
99: d8 d9                   fcomp  st(1)            // 0 ? edge2, edge2; x; y2; z;
9b: df e0                   fnstsw ax
9d: d0 ec                   shr    ah,1
9f: 73 0b                   jae    0xac             // jump if 0 >= edge2

a1: d8 e1                   fsub   st,st(1)         // edge2 - x; x; y2; z;
a3: d9 e1                   fabs                    // |edge2 - x|; x; y2; z;
a5: d9 e0                   fchs                    // -|edge2 - x|; x; y2; z;
a7: dc 46 80                fadd   edge2            // edge2 - |edge2 - x|; x; y2; z;
aa: d9 c9                   fxch   st(1)            // x; x2 = edge2 - |edge2 - x|; y2; z;

ac: dd d8                   fstp   st(0)            // x2; y2; z;

ae: 90                      nop
af: 90                      nop
b0: d9 d0                   fnop

v = rotation2 * v

b2: d9 c0                   fld    st(0)            // t.x2; t.x2; t.y2; t.z;
b4: d8 4e 98                fmul   rotation2.z.x    // rotation2.z.x * t.x2; t.x2; t.y2; t.z;
b7: d9 c2                   fld    st(2)            // t.y2; rotation2.z.x * t.x2; t.x2; t.y2; t.z;
b9: d8 4e 94                fmul   rotation2.z.y    // rotation2.z.y * t.y2; rotation2.z.x * t.x2; t.x2; t.y2; t.z;
bc: de c1                   faddp  st(1),st         // rotation2.z.x * t.x2 + rotation2.z.y * t.y2; t.x2; t.y2; t.z;
be: d9 c3                   fld    st(3)            // t.z; rotation2.z.x * t.x2 + rotation2.z.y * t.y2; t.x2; t.y2; t.z;
c0: d8 4e 90                fmul   rotation2.z.z    // rotation2.z.z * t.z; rotation2.z.x * t.x2 + rotation2.z.y * t.y2; t.x2; t.y2; t.z;
c3: de c1                   faddp  st(1),st         // dot(rotation2.z,t); t.x2; t.y2; t.z;
c5: d9 c1                   fld    st(1)            // t.x2; dot(rotation2.z,t); t.x2; t.y2; t.z;
c7: d8 4e b0                fmul   rotation2.x.x    // rotation2.x.x * t.x2; dot(rotation2.z,t); t.x2; t.y2; t.z;
ca: d9 c3                   fld    st(3)            // t.y2; rotation2.x.x * t.x2; dot(rotation2.z,t); t.x2; t.y2; t.z;
cc: d8 4e ac                fmul   rotation2.x.y    // rotation2.x.y * t.y2; rotation2.x.x * t.x2; dot(rotation2.z,t); t.x2; t.y2; t.z;
cf: de c1                   faddp  st(1),st         // rotation2.x.x * t.x2 + rotation2.x.y * t.y2; dot(rotation2.z,t); t.x2; t.y2; t.z;
d1: d9 c4                   fld    st(4)            // t.z; rotation2.x.x * t.x2 + rotation2.x.y * t.y2; dot(rotation2.z,t); t.x2; t.y2; t.z;
d3: d8 4e a8                fmul   rotation2.x.z    // rotation2.x.z * t.z; rotation2.x.x * t.x2 + rotation2.x.y * t.y2; dot(rotation2.z,t); t.x2; t.y2; t.z;
d6: de c1                   faddp  st(1),st         // dot(rotation2.x,t); dot(rotation2.z,t); t.x2; t.y2; t.z;
d8: d9 ca                   fxch   st(2)            // t.x2; dot(rotation2.z,t); dot(rotation2.x,t); t.y2; t.z;
da: d8 4e a4                fmul   rotation2.y.x    // rotation2.y.x * t.x2; dot(rotation2.z,t); dot(rotation2.x,t); t.y2; t.z;
dd: d9 cb                   fxch   st(3)            // t.y2; dot(rotation2.z,t); dot(rotation2.x,t); rotation2.y.x * t.x2; t.z;
df: d8 4e a0                fmul   rotation2.y.y    // rotation2.y.y * t.y2; dot(rotation2.z,t); dot(rotation2.x,t); rotation2.y.x * t.x2; t.z;
e2: de c3                   faddp  st(3),st         // dot(rotation2.z,t); dot(rotation2.x,t); rotation2.y.x * t.x2 + rotation2.y.y * t.y2; t.z;
e4: d9 cb                   fxch   st(3)            // t.z; dot(rotation2.x,t); rotation2.y.x * t.x2 + rotation2.y.y * t.y2; dot(rotation2.z,t);
e6: d8 4e 9c                fmul   rotation2.y.z    // rotation2.y.z * t.z; dot(rotation2.x,t); rotation2.y.x * t.x2 + rotation2.y.y * t.y2; dot(rotation2.z,t);
e9: de c2                   faddp  st(2),st         // dot(rotation2.x,t); dot(rotation2.y,t); dot(rotation2.z,t);

w *= scale

eb: dd 46 f0                fld    scale            // scale; dot(rotation2.x,t); dot(rotation2.y,t); dot(rotation2.z,t);
ee: dd 41 08                fld    w                // w; scale; dot(rotation2.x,t); dot(rotation2.y,t); dot(rotation2.z,t);
f1: d8 c9                   fmul   st,st(1)         // w * scale; scale; dot(rotation2.x,t); dot(rotation2.y,t); dot(rotation2.z,t);
f3: dd 59 08                fstp   w                // w *= scale, scale; dot(rotation2.x,t); dot(rotation2.y,t); dot(rotation2.z,t);

v - (scale - 1) * cscale

f6: dc cb                   fmul   st(3),st         // scale; dot(rotation2.x,t); dot(rotation2.y,t); scale * dot(rotation2.z,t);
f8: dc ca                   fmul   st(2),st         // scale; dot(rotation2.x,t); scale * dot(rotation2.y,t); scale * dot(rotation2.z,t);
fa: dc c9                   fmul   st(1),st         // scale; scale * dot(rotation2.x,t); scale * dot(rotation2.y,t); scale * dot(rotation2.z,t);
fc: d9 e8                   fld1                    // 1; scale; scale * dot(rotation2.x,t); scale * dot(rotation2.y,t); scale * dot(rotation2.z,t);
fe: de e9                   fsubp  st(1),st         // scale - 1; scale * dot(rotation2.x,t); scale * dot(rotation2.y,t); scale * dot(rotation2.z,t);
100:    d9 c0                   fld    st(0)        // scale - 1; scale - 1; scale * dot(rotation2.x,t); scale * dot(rotation2.y,t); scale * dot(rotation2.z,t);
102:    dc 4e d8                fmul   cscale.z     // (scale - 1) * cscale.z; scale - 1; scale * dot(rotation2.x,t); scale * dot(rotation2.y,t); scale * dot(rotation2.z,t);
105:    de ec                   fsubp  st(4),st     // scale - 1; scale * dot(rotation2.x,t); scale * dot(rotation2.y,t); scale * dot(rotation2.z,t) - (scale - 1) * cscale.z;
107:    d9 c0                   fld    st(0)        // scale - 1; scale - 1; scale * dot(rotation2.x,t); scale * dot(rotation2.y,t); scale * dot(rotation2.z,t) - (scale - 1) * cscale.z;
109:    dc 4e e0                fmul   cscale.y     // (scale - 1) * cscale.y; scale - 1; scale * dot(rotation2.x,t); scale * dot(rotation2.y,t); scale * dot(rotation2.z,t) - (scale - 1) * cscale.z;
10c:    de eb                   fsubp  st(3),st     // scale - 1; scale * dot(rotation2.x,t); scale * dot(rotation2.y,t) - (scale - 1) * cscale.y; scale * dot(rotation2.z,t) - (scale - 1) * cscale.z;
10e:    dc 4e e8                fmul   cscale.x     // (scale - 1) * cscale.x; scale * dot(rotation2.x,t); scale * dot(rotation2.y,t) - (scale - 1) * cscale.y; scale * dot(rotation2.z,t) - (scale - 1) * cscale.z;
111:    de e9                   fsubp  st(1),st     // scale * dot(rotation2.x,t) - (scale - 1) * cscale.x; scale * dot(rotation2.y,t) - (scale - 1) * cscale.y; scale * dot(rotation2.z,t) - (scale - 1) * cscale.z;
113:    dd 1b                   fstp   v.x          // v.x = scale * dot(rotation2.x,t) - (scale - 1) * cscale.x, scale * dot(rotation2.y,t) - (scale - 1) * cscale.y; scale * dot(rotation2.z,t) - (scale - 1) * cscale.z;
115:    dd 1a                   fstp   v.y          // v.y = scale * dot(rotation2.y,t) - (scale - 1) * cscale.y, scale * dot(rotation2.z,t) - (scale - 1) * cscale.z;
117:    dd 19                   fstp   v.z          // v.z = scale * dot(rotation2.z,t) - (scale - 1) * cscale.z

119:    89 d8                   mov    eax,ebx

11b:    5e                      pop    esi
11c:    5b                      pop    ebx

11d:    5d                      pop    ebp
11e:    c2 08 00                ret    0x8
```

## Code

```rust
v = rotation1 * v;
v.x = -v.x;
if v.x >= v.y { v = v.yxz; }
if v.x >= v.z { v = v.zyx; }
v.x = -v.x;
v = v.yxz;
v.x = -v.x;
if v.x >= v.z { v = v.zyx; }
v.x = -v.x;
v = v.yxz;
if (0 < edge1) { v.y = edge1 - (edge1 - v.y).abs() }
if (0 < edge2) { v.x = edge2 - (edge2 - v.x).abs() }
w *= scale;
v = rotation2 * v - (scale - 1) * cscale;
```
