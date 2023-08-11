# `Menger3`

```
[OPTIONS]
.Version = 2
.DEscale = 0.2
.DEoption = 2
.RStop = 1024
.Double Scale = 3
.Double CScale X = 1
.Double CScale Y = 1
.Double CScale Z = 1
.3SingleAngles Rotation1 = 0
.3SingleAngles Rotation2 = 0
[CODE]
558BEC535689C38B75088B7630B8FFFFFF7F214104214204214304DD01DD02DD
03D9C0D84EBCD9C2D84EB8DEC1D9C3D84EB4DEC1D9C1D84ED4D9C3D84ED0DEC1
D9C4D84ECCDEC1D9CAD84EC8D9CBD84EC4DEC3D9CBD84EC0DEC2D8D1DFE0D0EC
7302D9C9D8D2DFE0D0EC7302D9CAD9C9D8D2DFE0D0EC7302D9CAD9C1D84E98D9
C1D84E94DEC1D9C3D84E90DEC1D9C2D84EB0D9C2D84EACDEC1D9C4D84EA8DEC1
D9CBD84EA4D9CAD84EA0DEC2D9CBD84E9CDEC1DD46F0DD4108D8C9DD5908DCCB
DCCADCC9D9E8DEE9D9C0D9C0DC4EE8DEECDC4EE0DEEADC4ED8D9C0DC4EF8D8DC
DFE080E4417402DCEBDDD8DD1ADD1BDD1989D85E5B5DC20800
[END]
```

## Disassembly

```
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
```

```
0:  55                      push   ebp
1:  8b ec                   mov    ebp,esp

3:  53                      push   ebx
4:  56                      push   esi

5:  89 c3                   mov    ebx,eax

7:  8b 75 08                mov    esi,DWORD PTR [ebp+0x8]
a:  8b 76 30                mov    esi,DWORD PTR [esi+0x30]

// v = abs(v)

d:  b8 ff ff ff 7f          mov    eax,0x7fffffff
12: 21 41 04                and    DWORD PTR [ecx+0x4],eax
15: 21 42 04                and    DWORD PTR [edx+0x4],eax
18: 21 43 04                and    DWORD PTR [ebx+0x4],eax

// v = rotation1 * v

1b: dd 01                   fld    z               // z;
1d: dd 02                   fld    y               // y; z;
1f: dd 03                   fld    x               // x; y; z;

21: d9 c0                   fld    st(0)           // x; x; y; z;
23: d8 4e bc                fmul   rotation1.z.x   // rotation1.z.x * x; x; y; z;
26: d9 c2                   fld    st(2)           // y; rotation1.z.x * x; x; y; z;
28: d8 4e b8                fmul   rotation1.z.y   // rotation1.z.y * y; rotation1.z.x * x; x; y; z;
2b: de c1                   faddp  st(1),st        // rotation1.z.x * x + rotation1.z.y * y; x; y; z;
2d: d9 c3                   fld    st(3)           // z; rotation1.z.x * x + rotation1.z.y * y; x; y; z;
2f: d8 4e b4                fmul   rotation1.z.z   // rotation1.z.z * z; rotation1.z.x * x + rotation1.z.y * y; x; y; z;
32: de c1                   faddp  st(1),st        // (rotation1 * v).z; x; y; z;

34: d9 c1                   fld    st(1)           // x; (rotation1 * v).z; x; y; z;
36: d8 4e d4                fmul   rotation1.x.x   // rotation1.x.x * x; (rotation1 * v).z; x; y; z;
39: d9 c3                   fld    st(3)           // y; rotation1.x.x * x; (rotation1 * v).z; x; y; z;
3b: d8 4e d0                fmul   rotation1.x.y   // rotation1.x.y * y; rotation1.x.x * x; (rotation1 * v).z; x; y; z;
3e: de c1                   faddp  st(1),st        // rotation1.x.x * x + rotation1.x.y * y; (rotation1 * v).z; x; y; z;
40: d9 c4                   fld    st(4)           // z; rotation1.x.x * x + rotation1.x.y * y; (rotation1 * v).z; x; y; z;
42: d8 4e cc                fmul   rotation1.x.z   // rotation1.x.z * z; rotation1.x.x * x + rotation1.x.y * y; (rotation1 * v).z; x; y; z;
45: de c1                   faddp  st(1),st        // (rotation1 * v).x; (rotation1 * v).z; x; y; z;

47: d9 ca                   fxch   st(2)           // x; (rotation1 * v).z; (rotation1 * v).x; y; z;
49: d8 4e c8                fmul   rotation1.y.x   // rotation1.y.x * x; (rotation1 * v).z; (rotation1 * v).x; y; z;
4c: d9 cb                   fxch   st(3)           // y; (rotation1 * v).z; (rotation1 * v).x; rotation1.y.x * x; z;
4e: d8 4e c4                fmul   rotation1.y.y   // rotation1.y.y * y; (rotation1 * v).z; (rotation1 * v).x; rotation1.y.x * x; z;
51: de c3                   faddp  st(3),st        // (rotation1 * v).z; (rotation1 * v).x; rotation1.y.x * x + rotation1.y.y * y; z;
53: d9 cb                   fxch   st(3)           // z; (rotation1 * v).x; rotation1.y.x * x + rotation1.y.y * y; (rotation1 * v).z;
55: d8 4e c0                fmul   rotation1.y.z   // rotation1.y.z * z; (rotation1 * v).x; rotation1.y.x * x + rotation1.y.y * y; (rotation1 * v).z;
58: de c2                   faddp  st(2),st        // (rotation1 * v).x; (rotation1 * v).y; (rotation1 * v).z;

// if (x < y) swap x and y

5a: d8 d1                   fcom   st(1)           // x >= y, x; y; z;
5c: df e0                   fnstsw ax
5e: d0 ec                   shr    ah,1
60: 73 02                   jae    0x64

62: d9 c9                   fxch   st(1)           // if x < y, swap x and y

// if (x < z) swap x and z

64: d8 d2                   fcom   st(2)           // x >= z, x; y; z;
66: df e0                   fnstsw ax
68: d0 ec                   shr    ah,1
6a: 73 02                   jae    0x6e

6c: d9 ca                   fxch   st(2)           // if x < z, swap x and z

// if (y < z) swap y and z

6e: d9 c9                   fxch   st(1)           // y; x; z;
70: d8 d2                   fcom   st(2)           // y >= z, y; x; z;
72: df e0                   fnstsw ax
74: d0 ec                   shr    ah,1
76: 73 02                   jae    0x7a

78: d9 ca                   fxch   st(2)           // if y < z, swap y and z

// v = rotation2 * v

7a: d9 c1                   fld    st(1)           // x; y; x; z;
7c: d8 4e 98                fmul   rotation2.z.x   // rotation2.z.x * x; y; x; z;
7f: d9 c1                   fld    st(1)           // y; rotation2.z.x * x; y; x; z;
81: d8 4e 94                fmul   rotation2.z.y   // rotation2.z.y * y; rotation2.z.x * x; y; x; z;
84: de c1                   faddp  st(1),st        // rotation2.z.x * x + rotation2.z.y * y; y; x; z;
86: d9 c3                   fld    st(3)           // z; rotation2.z.x * x + rotation2.z.y * y; y; x; z;
88: d8 4e 90                fmul   rotation2.z.z   // rotation2.z.z * z; rotation2.z.x * x + rotation2.z.y * y; y; x; z;
8b: de c1                   faddp  st(1),st        // (rotation2 * v).z; y; x; z;

8d: d9 c2                   fld    st(2)           // x; (rotation2 * v).z; y; x; z;
8f: d8 4e b0                fmul   rotation2.x.x   // rotation2.x.x * x; (rotation2 * v).z; y; x; z;
92: d9 c2                   fld    st(2)           // y; rotation2.x.x * x; (rotation2 * v).z; y; x; z;
94: d8 4e ac                fmul   rotation2.x.y   // rotation2.x.y * y; rotation2.x.x * x; (rotation2 * v).z; y; x; z;
97: de c1                   faddp  st(1),st        // rotation2.x.x * x + rotation2.x.y * y; (rotation2 * v).z; y; x; z;
99: d9 c4                   fld    st(4)           // z; rotation2.x.x * x + rotation2.x.y * y; (rotation2 * v).z; y; x; z;
9b: d8 4e a8                fmul   rotation2.x.z   // rotation2.x.z * z; rotation2.x.x * x + rotation2.x.y * y; (rotation2 * v).z; y; x; z;
9e: de c1                   faddp  st(1),st        // (rotation2 * v).x; (rotation2 * v).z; y; x; z;

a0: d9 cb                   fxch   st(3)           // x; (rotation2 * v).z; y; (rotation2 * v).x; z;
a2: d8 4e a4                fmul   rotation2.y.x   // rotation2.y.x * x; (rotation2 * v).z; y; (rotation2 * v).x; z;
a5: d9 ca                   fxch   st(2)           // y; (rotation2 * v).z; rotation2.y.x * x; (rotation2 * v).x; z;
a7: d8 4e a0                fmul   rotation2.y.y   // rotation2.y.y * y; (rotation2 * v).z; rotation2.y.x * x; (rotation2 * v).x; z;
aa: de c2                   faddp  st(2),st        // (rotation2 * v).z; rotation2.y.x * x + rotation2.y.y * y; (rotation2 * v).x; z;
ac: d9 cb                   fxch   st(3)           // z; rotation2.y.x * x + rotation2.y.y * y; (rotation2 * v).x; (rotation2 * v).z;
ae: d8 4e 9c                fmul   rotation2.y.z   // rotation2.y.z * z; rotation2.y.x * x + rotation2.y.y * y; (rotation2 * v).x; (rotation2 * v).z;
b1: de c1                   faddp  st(1),st        // (rotation2 * v).y; (rotation2 * v).x; (rotation2 * v).z;

// w = w * scale

b3: dd 46 f0                fld    scale           // scale; y; x; z;
b6: dd 41 08                fld    w               // w; scale; y; x; z;
b9: d8 c9                   fmul   st,st(1)        // w * scale; scale; y; x; z;
bb: dd 59 08                fstp   w               // w = w * scale, scale; y; x; z;

// v *= scale

be: dc cb                   fmul   st(3),st        // scale; y; x; z * scale;
c0: dc ca                   fmul   st(2),st        // scale; y; x * scale; z * scale;
c2: dc c9                   fmul   st(1),st        // scale; y * scale; x * scale; z * scale;

// cs = cscale * (scale - 1)
// v.x -= cs.x
// v.y -= cs.y

c4: d9 e8                   fld1                   // 1; scale; y; x; z;
c6: de e9                   fsubp  st(1),st        // scale - 1; y; x; z;
c8: d9 c0                   fld    st(0)           // scale - 1; scale - 1; y; x; z;
ca: d9 c0                   fld    st(0)           // scale - 1; scale - 1; scale - 1; y; x; z;
cc: dc 4e e8                fmul   cscale.x        // cscale.x * (scale - 1); scale - 1; scale - 1; y; x; z;
cf: de ec                   fsubp  st(4),st        // scale - 1; scale - 1; y; x - cscale.x * (scale - 1); z;
d1: dc 4e e0                fmul   cscale.y        // cscale.y * (scale - 1); scale - 1; y; x - cscale.x * (scale - 1); z;
d4: de ea                   fsubp  st(2),st        // scale - 1; y - cscale.y * (scale - 1); x - cscale.x * (scale - 1); z;
d6: dc 4e d8                fmul   cscale.z        // cscale.z * (scale - 1); y - cscale.y * (scale - 1); x - cscale.x * (scale - 1); z;

// if (z > 0.5 * cs.z) v.z -= cs.z

d9: d9 c0                   fld    st(0)           // cscale.z * (scale - 1); cscale.z * (scale - 1); y - cscale.y * (scale - 1); x - cscale.x * (scale - 1); z;
db: dc 4e f8                fmul   0.5             // 0.5 * cscale.z * (scale - 1); cscale.z * (scale - 1); y - cscale.y * (scale - 1); x - cscale.x * (scale - 1); z;
de: d8 dc                   fcomp  st(4)           // 0.5 * cscale.z * (scale - 1) ? z, cscale.z * (scale - 1); y - cscale.y * (scale - 1); x - cscale.x * (scale - 1); z;
e0: df e0                   fnstsw ax
e2: 80 e4 41                and    ah,0x41
e5: 74 02                   je     0xe9            // jump if 0.5 * cscale.z * (scale - 1) > z

e7: dc eb                   fsub   st(3),st        // cscale.z * (scale - 1); y - cscale.y * (scale - 1); x - cscale.x * (scale - 1); z - cscale.z * (scale - 1);

e9: dd d8                   fstp   st(0)           // y - cscale.y * (scale - 1); x - cscale.x * (scale - 1); z - cscale.z * (scale - 1);

eb: dd 1a                   fstp   y
ed: dd 1b                   fstp   x
ef: dd 19                   fstp   z

f1: 89 d8                   mov    eax,ebx

f3: 5e                      pop    esi
f4: 5b                      pop    ebx

f5: 5d                      pop    ebp
f6: c2 08 00                ret    0x8
```

## Code

```glsl
v = abs(v);
v = rotation1 * v;
if (x < y) { v = v.yxz; }
if (x < z) { v = v.zyx; }
if (y < z) { v = v.xzy; }
v = rotation2 * v;
w = w * scale;
v *= scale;
cs = cscale * (scale - 1);
v.x -= cs.x;
v.y -= cs.y;
if (z > 0.5 * cs.z) { v.z -= cs.z }
```
