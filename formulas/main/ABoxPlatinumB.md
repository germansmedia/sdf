# `ABoxPlatinumB`

```
[OPTIONS]
.Version = 4
.DEscale = 0.2
.DEoption = 2
.RStop = 1024
.SIpower = 2
.Double Scale = 2
.Boxscale MinR/IR = 0.5
.Double Fold = 1
.3SingleAngles Rotate = 0
.Double Inv xC = 0
.Double Inv yC = 0
.Double Inv zC = 0
.DRecipro Inv Radius = 1
.DoubleAngle FoldX, XY angle = 0
.DoubleAngle FoldX, XZ angle = 0
.DoubleAngle FoldY, XY angle = 0
.DoubleAngle FoldY, YZ angle = 0
.Integer Abs XYZ switches = 0
[CODE]
558BEC56538B750889C38B763083C4D0DD46D8DD03DC8E74FFFFFFDD01DC8E7C
FFFFFFDEE9DD1C24DD03DC8E7CFFFFFFDD01DC8E74FFFFFFDEC1DD5C24089090
DD0424DC4E84DD02DC4E8CDEE990DD0424DC4E8CDD02DC4E84DEC1DD5C241090
D9C0D8E2D9E1D8C1D9C9D8C2D9E1DEE1DC4E84DD442410DC4E8CDEE990DC8E74
FFFFFFDD442408DC8E7CFFFFFFDEE990DD02DC8E54FFFFFFDD01DC8E5CFFFFFF
DEE9DD1C24DD02DC8E5CFFFFFFDD01DC8E54FFFFFFDEC1DD5C24089090DD0424
DC8E64FFFFFFDD03DC8E6CFFFFFFDEE990DD0424DC8E6CFFFFFFDD03DC8E64FF
FFFFDEC1DD5C2410D9C0D8E3D9E1D8C1D9C9D8C3D9E1DEE1DC8E64FFFFFF90DD
442410DC8E6CFFFFFFDEE990DC8E54FFFFFFDD442408DC8E5CFFFFFFDEE9DD01
D9C0D8E4D9E1D8C1D9C9D8C4D9E1DEE1908B8650FFFFFF83E0047402D9E18B86
50FFFFFF83E0027406D9C9D9E1D9C98B8650FFFFFF83E0017406D9CAD9E1D9CA
90DD469CDCC1DDD890DD46A4DCC2DDD890DD46ACDCC3DDD8D9C0D8C8D9C2D8C8
DEC1D9C3D8C8DEC1DC4E9490DC56E0DFE0D0EC7307DDD8DD46E8EB16D9E8D8D1
DFE0D0EC7207DDD8DC7EF0EB05DED9DD46F0DD4108D8C9DD5908DCCBDCCADEC9
90DD469CDC4EF0DCE9DDD890DD46A4DC4EF0DCEADDD890DD46ACDC4EF0DCEBDD
D890D9CAD9C0D84EBCD9C2D84EB8DEC1D9C3D84EB4DEC1D9C1D84ED4D9C3D84E
D0DEC1D9C4D84ECCDEC1D9CAD84EC8D9CBD84EC4DEC3D9CBD84EC0DEC2D9CA8B
7508DC4628DD19DC4620DD1ADC4618DD1BDDD883C43089D85B5E5DC20800
[END]

'Platinum' edition is inspired by Buddhi's version. Lots of added features, but slower.
X and Y folds are rotatable, but due to an approximation in the formula the result is distorted.
The result will NOT be infinitely zoomable with nice results, but it's interesting anyway.
I decided to leave it as a variant.
If Fold angles are zero, this is absolutely identical to the normal version!

Luca GN 2012
```

## Disassembly

[ebx] = v.x
[edx] = v.y
[ecx] = v.z

[esp]
[esp+0x8]
[esp+0x10]

[ebp+0x8]

[esi+0x18]
[esi+0x20]
[esi+0x28]

.Double Scale = 2
[esi-0x10]

.Boxscale MinR/IR = 0.5
[esi-0x18] scale/sqr(minr)
[esi-0x20] sqr(minr)

.Double Fold = 1
[esi-0x28]

.3SingleAngles Rotate = 0
[esi-0x2c] rot.x.x
[esi-0x30] rot.x.y
[esi-0x34] rot.x.z
[esi-0x38] rot.y.x
[esi-0x3c] rot.y.y
[esi-0x40] rot.y.z
[esi-0x44] rot.z.x
[esi-0x48] rot.z.y
[esi-0x4c] rot.z.z

.Double Inv xC = 0
[esi-0x54]

.Double Inv yC = 0
[esi-0x5c]

.Double Inv zC = 0
[esi-0x64]

.DRecipro Inv Radius = 1
[esi-0x6c]

.DoubleAngle FoldX, XY angle = 0
[esi-0x74] fx_sinxy
[esi-0x7c] fx_cosxy

.DoubleAngle FoldX, XZ angle = 0
[esi-0x84] fx_sinxz
[esi-0x8c] fx_cosxz

.DoubleAngle FoldY, XY angle = 0
[esi-0x94] fy_sinxy
[esi-0x9c] fy_cosxy

.DoubleAngle FoldY, YZ angle = 0
[esi-0xa4] fy_sinyz
[esi-0xac] fy_cosyz

.Integer Abs XYZ switches = 0
[esi-0xb0]

```
0:  55                      push   ebp
1:  8b ec                   mov    ebp,esp

3:  56                      push   esi
4:  53                      push   ebx
5:  8b 75 08                mov    esi,DWORD PTR [ebp+0x8]
8:  89 c3                   mov    ebx,eax
a:  8b 76 30                mov    esi,DWORD PTR [esi+0x30]

d:  83 c4 d0                add    esp,0xffffffd0

10: dd 46 d8                fld    fold           // fold;
13: dd 03                   fld    v.x            // v.x; fold;
15: dc 8e 74 ff ff ff       fmul   fx_sinxz       // v.x * fx_sinxz; fold;
1b: dd 01                   fld    v.z            // v.z; v.x * fx_sinxz; fold;
1d: dc 8e 7c ff ff ff       fmul   fx_cosxz       // v.z * fx_cosxz; v.x * fx_sinxz; fold;
23: de e9                   fsubp  st(1),st       // v.x * fx_sinxz - v.z * fx_cosxz; fold;
25: dd 1c 24                fstp   t0             // t0 = v.x * fx_sinxz - v.z * fx_cosxz, fold;

28: dd 03                   fld    v.x            // v.x; fold;
2a: dc 8e 7c ff ff ff       fmul   fx_cosxy       // v.x * fx_cosxy; fold;
30: dd 01                   fld    v.z            // v.z; v.x * fx_cosxy; fold;
32: dc 8e 74 ff ff ff       fmul   fx_sinxy       // v.z * fx_sinxy; v.x * fx_cosxy; fold;
38: de c1                   faddp  st(1),st       // v.x * fx_cosxy + v.z * fx_sinxy; fold;
3a: dd 5c 24 08             fstp   t1             // t1 = v.x * fx_cosxy + v.z * fx_sinxy, fold;

3e: 90                      nop
3f: 90                      nop

40: dd 04 24                fld    t0             // t0; fold;
43: dc 4e 84                fmul   fx_sinxz       // t0 * fx_sinxz; fold;
46: dd 02                   fld    v.y            // v.y; t0 * fx_sinxz; fold;
48: dc 4e 8c                fmul   fx_cosxz       // v.y * fx_cosxz; t0 * fx_sinxz; fold;
4b: de e9                   fsubp  st(1),st       // t0 * fx_sinxz - v.y * fx_cosxz; fold;

4d: 90                      nop

4e: dd 04 24                fld    t0             // t0; t0 * fx_sinxz - v.y * fx_cosxz; fold;
51: dc 4e 8c                fmul   fx_cosxz       // t0 * fx_cosxz; t0 * fx_sinxz - v.y * fx_cosxz; fold;
54: dd 02                   fld    v.y            // v.y; t0 * fx_cosxz; t0 * fx_sinxz - v.y * fx_cosxz; fold;
56: dc 4e 84                fmul   fx_sinxz       // v.y * fx_sinxz; t0 * fx_cosxz; t0 * fx_sinxz - v.y * fx_cosxz; fold;
59: de c1                   faddp  st(1),st       // t0 * fx_cosxz + v.y * fx_sinxz; t0 * fx_sinxz - v.y * fx_cosxz; fold;
5b: dd 5c 24 10             fstp   t2             // t2 = t0 * fx_cosxz + v.y * fx_sinxz, t0 * fx_sinxz - v.y * fx_cosxz; fold;

5f: 90                      nop

60: d9 c0                   fld    st(0)          // t0 * fx_sinxz - v.y * fx_cosxz; t0 * fx_sinxz - v.y * fx_cosxz; fold;
62: d8 e2                   fsub   st,st(2)       // t0 * fx_sinxz - v.y * fx_cosxz - fold; t0 * fx_sinxz - v.y * fx_cosxz; fold;
64: d9 e1                   fabs                  // |t0 * fx_sinxz - v.y * fx_cosxz - fold|; t0 * fx_sinxz - v.y * fx_cosxz; fold;
66: d8 c1                   fadd   st,st(1)       // |t0 * fx_sinxz - v.y * fx_cosxz - fold| + t0 * fx_sinxz - v.y * fx_cosxz; t0 * fx_sinxz - v.y * fx_cosxz; fold;
68: d9 c9                   fxch   st(1)          // t0 * fx_sinxz - v.y * fx_cosxz; |t0 * fx_sinxz - v.y * fx_cosxz - fold| + t0 * fx_sinxz - v.y * fx_cosxz; fold;
6a: d8 c2                   fadd   st,st(2)       // t0 * fx_sinxz - v.y * fx_cosxz + fold; |t0 * fx_sinxz - v.y * fx_cosxz - fold| + t0 * fx_sinxz - v.y * fx_cosxz; fold;
6c: d9 e1                   fabs                  // |t0 * fx_sinxz - v.y * fx_cosxz + fold|; |t0 * fx_sinxz - v.y * fx_cosxz - fold| + t0 * fx_sinxz - v.y * fx_cosxz; fold;
6e: de e1                   fsubrp st(1),st       // |t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz; fold;
70: dc 4e 84                fmul   fx_sinxz       // (|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz; fold;
73: dd 44 24 10             fld    t2             // t2; (|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz; fold;
77: dc 4e 8c                fmul   fx_cosxz       // t2 * fx_cosxz; (|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz; fold;
7a: de e9                   fsubp  st(1),st       // (|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz; fold;

7c: 90                      nop

7d: dc 8e 74 ff ff ff       fmul   fx_sinxy       // ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy; fold;
83: dd 44 24 08             fld    t1             // t1; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy; fold;
87: dc 8e 7c ff ff ff       fmul   fx_cosxy       // t1 * fx_cosxy; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy; fold;
8d: de e9                   fsubp  st(1),st       // ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;

8f: 90                      nop

90: dd 02                   fld    v.y            // v.y; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
92: dc 8e 54 ff ff ff       fmul   fy_cosyz       // v.y * fy_cosyz; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
98: dd 01                   fld    v.z            // v.z; v.y * fy_cosyz; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
9a: dc 8e 5c ff ff ff       fmul   fy_sinyz       // v.z * fy_sinyz; v.y * fy_cosyz; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
a0: de e9                   fsubp  st(1),st       // v.y * fy_cosyz - v.z * fy_sinyz; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
a2: dd 1c 24                fstp   t0             // t0 = v.y * fy_cosyz - v.z * fy_sinyz, ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;

a5: dd 02                   fld    v.y            // v.y; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
a7: dc 8e 5c ff ff ff       fmul   fy_sinyz       // v.y * fy_sinyz; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
ad: dd 01                   fld    v.z            // v.z; v.y * fy_sinyz; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
af: dc 8e 54 ff ff ff       fmul   fy_cosyz       // v.z * fy_cosyz; v.y * fy_sinyz; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
b5: de c1                   faddp  st(1),st       // v.y * fy_sinyz + v.z * fy_cosyz; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
b7: dd 5c 24 08             fstp   t1             // t1 = v.y * fy_sinyz + v.z * fy_cosyz, ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;

bb: 90                      nop
bc: 90                      nop

bd: dd 04 24                fld    t0             // t0; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
c0: dc 8e 64 ff ff ff       fmul   fy_cosxy       // t0 * fy_cosxy; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
c6: dd 03                   fld    v.x            // v.x; t0 * fy_cosxy; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
c8: dc 8e 6c ff ff ff       fmul   fy_sinxy       // v.x * fy_sinxy; t0 * fy_cosxy; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
ce: de e9                   fsubp  st(1),st       // t0 * fy_cosxy - v.x * fy_sinxy; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;

d0: 90                      nop

d1: dd 04 24                fld    t0             // t0; t0 * fy_cosxy - v.x * fy_sinxy; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
d4: dc 8e 6c ff ff ff       fmul   fy_sinxy       // t0 * fy_sinxy; t0 * fy_cosxy - v.x * fy_sinxy; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
da: dd 03                   fld    v.x            // v.x; t0 * fy_sinxy; t0 * fy_cosxy - v.x * fy_sinxy; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
dc: dc 8e 64 ff ff ff       fmul   fy_cosxy       // v.x * fy_cosxy; t0 * fy_sinxy; t0 * fy_cosxy - v.x * fy_sinxy; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
e2: de c1                   faddp  st(1),st       // t0 * fy_sinxy + v.x * fy_cosxy; t0 * fy_cosxy - v.x * fy_sinxy; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
e4: dd 5c 24 10             fstp   t2             // t2 = t0 * fy_sinxy + v.x * fy_cosxy, t0 * fy_cosxy - v.x * fy_sinxy; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
e8: d9 c0                   fld    st(0)          // t0 * fy_cosxy - v.x * fy_sinxy; t0 * fy_cosxy - v.x * fy_sinxy; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
ea: d8 e3                   fsub   st,st(3)       // t0 * fy_cosxy - v.x * fy_sinxy - fold; t0 * fy_cosxy - v.x * fy_sinxy; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
ec: d9 e1                   fabs                  // |t0 * fy_cosxy - v.x * fy_sinxy - fold|; t0 * fy_cosxy - v.x * fy_sinxy; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
ee: d8 c1                   fadd   st,st(1)       // |t0 * fy_cosxy - v.x * fy_sinxy - fold| + t0 * fy_cosxy - v.x * fy_sinxy; t0 * fy_cosxy - v.x * fy_sinxy; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
f0: d9 c9                   fxch   st(1)          // t0 * fy_cosxy - v.x * fy_sinxy; |t0 * fy_cosxy - v.x * fy_sinxy - fold| + t0 * fy_cosxy - v.x * fy_sinxy; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
f2: d8 c3                   fadd   st,st(3)       // t0 * fy_cosxy - v.x * fy_sinxy + fold; |t0 * fy_cosxy - v.x * fy_sinxy - fold| + t0 * fy_cosxy - v.x * fy_sinxy; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
f4: d9 e1                   fabs                  // |t0 * fy_cosxy - v.x * fy_sinxy + fold|; |t0 * fy_cosxy - v.x * fy_sinxy - fold| + t0 * fy_cosxy - v.x * fy_sinxy; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
f6: de e1                   fsubrp st(1),st       // |t0 * fy_cosxy - v.x * fy_sinxy + fold| - (|t0 * fy_cosxy - v.x * fy_sinxy - fold| + t0 * fy_cosxy - v.x * fy_sinxy); ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
f8: dc 8e 64 ff ff ff       fmul   fy_cosxy       // (|t0 * fy_cosxy - v.x * fy_sinxy + fold| - (|t0 * fy_cosxy - v.x * fy_sinxy - fold| + t0 * fy_cosxy - v.x * fy_sinxy)) * fy_cosxy; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;

fe: 90                      nop

ff: dd 44 24 10             fld    t2             // t2; (|t0 * fy_cosxy - v.x * fy_sinxy + fold| - (|t0 * fy_cosxy - v.x * fy_sinxy - fold| + t0 * fy_cosxy - v.x * fy_sinxy)) * fy_cosxy; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
103:    dc 8e 6c ff ff ff       fmul   fy_sinxy   // t2 * fy_sinxy; (|t0 * fy_cosxy - v.x * fy_sinxy + fold| - (|t0 * fy_cosxy - v.x * fy_sinxy - fold| + t0 * fy_cosxy - v.x * fy_sinxy)) * fy_cosxy; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
109:    de e9                   fsubp  st(1),st   // (|t0 * fy_cosxy - v.x * fy_sinxy + fold| - (|t0 * fy_cosxy - v.x * fy_sinxy - fold| + t0 * fy_cosxy - v.x * fy_sinxy)) * fy_cosxy - t2 * fy_sinxy; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;

10b:    90                      nop

10c:    dc 8e 54 ff ff ff       fmul   fy_cosyz   // ((|t0 * fy_cosxy - v.x * fy_sinxy + fold| - (|t0 * fy_cosxy - v.x * fy_sinxy - fold| + t0 * fy_cosxy - v.x * fy_sinxy)) * fy_cosxy - t2 * fy_sinxy) * fy_cosyz; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
112:    dd 44 24 08             fld    t1         // t1; ((|t0 * fy_cosxy - v.x * fy_sinxy + fold| - (|t0 * fy_cosxy - v.x * fy_sinxy - fold| + t0 * fy_cosxy - v.x * fy_sinxy)) * fy_cosxy - t2 * fy_sinxy) * fy_cosyz; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
116:    dc 8e 5c ff ff ff       fmul   fy_sinyz   // t1 * fy_sinyz; ((|t0 * fy_cosxy - v.x * fy_sinxy + fold| - (|t0 * fy_cosxy - v.x * fy_sinxy - fold| + t0 * fy_cosxy - v.x * fy_sinxy)) * fy_cosxy - t2 * fy_sinxy) * fy_cosyz; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
11c:    de e9                   fsubp  st(1),st   // ((|t0 * fy_cosxy - v.x * fy_sinxy + fold| - (|t0 * fy_cosxy - v.x * fy_sinxy - fold| + t0 * fy_cosxy - v.x * fy_sinxy)) * fy_cosxy - t2 * fy_sinxy) * fy_cosyz - t1 * fy_sinyz; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
11e:    dd 01                   fld    v.z        // v.z; ((|t0 * fy_cosxy - v.x * fy_sinxy + fold| - (|t0 * fy_cosxy - v.x * fy_sinxy - fold| + t0 * fy_cosxy - v.x * fy_sinxy)) * fy_cosxy - t2 * fy_sinxy) * fy_cosyz - t1 * fy_sinyz; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
120:    d9 c0                   fld    st(0)      // v.z; v.z; ((|t0 * fy_cosxy - v.x * fy_sinxy + fold| - (|t0 * fy_cosxy - v.x * fy_sinxy - fold| + t0 * fy_cosxy - v.x * fy_sinxy)) * fy_cosxy - t2 * fy_sinxy) * fy_cosyz - t1 * fy_sinyz; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
122:    d8 e4                   fsub   st,st(4)   // v.z - fold; v.z; ((|t0 * fy_cosxy - v.x * fy_sinxy + fold| - (|t0 * fy_cosxy - v.x * fy_sinxy - fold| + t0 * fy_cosxy - v.x * fy_sinxy)) * fy_cosxy - t2 * fy_sinxy) * fy_cosyz - t1 * fy_sinyz; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
124:    d9 e1                   fabs              // |v.z - fold|; v.z; ((|t0 * fy_cosxy - v.x * fy_sinxy + fold| - (|t0 * fy_cosxy - v.x * fy_sinxy - fold| + t0 * fy_cosxy - v.x * fy_sinxy)) * fy_cosxy - t2 * fy_sinxy) * fy_cosyz - t1 * fy_sinyz; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
126:    d8 c1                   fadd   st,st(1)   // |v.z - fold| + v.z; v.z; ((|t0 * fy_cosxy - v.x * fy_sinxy + fold| - (|t0 * fy_cosxy - v.x * fy_sinxy - fold| + t0 * fy_cosxy - v.x * fy_sinxy)) * fy_cosxy - t2 * fy_sinxy) * fy_cosyz - t1 * fy_sinyz; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
128:    d9 c9                   fxch   st(1)      // v.z; |v.z - fold| + v.z; ((|t0 * fy_cosxy - v.x * fy_sinxy + fold| - (|t0 * fy_cosxy - v.x * fy_sinxy - fold| + t0 * fy_cosxy - v.x * fy_sinxy)) * fy_cosxy - t2 * fy_sinxy) * fy_cosyz - t1 * fy_sinyz; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
12a:    d8 c4                   fadd   st,st(4)   // v.z + fold; |v.z - fold| + v.z; ((|t0 * fy_cosxy - v.x * fy_sinxy + fold| - (|t0 * fy_cosxy - v.x * fy_sinxy - fold| + t0 * fy_cosxy - v.x * fy_sinxy)) * fy_cosxy - t2 * fy_sinxy) * fy_cosyz - t1 * fy_sinyz; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
12c:    d9 e1                   fabs              // |v.z + fold|; |v.z - fold| + v.z; ((|t0 * fy_cosxy - v.x * fy_sinxy + fold| - (|t0 * fy_cosxy - v.x * fy_sinxy - fold| + t0 * fy_cosxy - v.x * fy_sinxy)) * fy_cosxy - t2 * fy_sinxy) * fy_cosyz - t1 * fy_sinyz; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;
12e:    de e1                   fsubrp st(1),st   // |v.z + fold| - |v.z - fold| - v.z; ((|t0 * fy_cosxy - v.x * fy_sinxy + fold| - (|t0 * fy_cosxy - v.x * fy_sinxy - fold| + t0 * fy_cosxy - v.x * fy_sinxy)) * fy_cosxy - t2 * fy_sinxy) * fy_cosyz - t1 * fy_sinyz; ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy; fold;

// some renaming:
// t.z = |v.z + fold| - |v.z - fold| - v.z
// t.y = ((|t0 * fy_cosxy - v.x * fy_sinxy + fold| - (|t0 * fy_cosxy - v.x * fy_sinxy - fold| + t0 * fy_cosxy - v.x * fy_sinxy)) * fy_cosxy - t2 * fy_sinxy) * fy_cosyz - t1 * fy_sinyz
// t.x = ((|t0 * fx_sinxz - v.y * fx_cosxz + fold| - |t0 * fx_sinxz - v.y * fx_cosxz - fold| - t0 * fx_sinxz + v.y * fx_cosxz) * fx_sinxz - t2 * fx_cosxz) * fx_sinxy - t1 * fx_cosxy

// fpu stack: t.z; t.y; t.x; fold;

130:    90                      nop

// the switch bits enable fabs on st, st(1) and/or st(2)
131:    8b 86 50 ff ff ff       mov    eax,switches
137:    83 e0 04                and    eax,0x4    // switches & 0x04
13a:    74 02                   je     0x13e

13c:    d9 e1                   fabs

13e:    8b 86 50 ff ff ff       mov    eax,switches
144:    83 e0 02                and    eax,0x2    // switches & 0x02
147:    74 06                   je     0x14f

149:    d9 c9                   fxch   st(1)
14b:    d9 e1                   fabs
14d:    d9 c9                   fxch   st(1)

14f:    8b 86 50 ff ff ff       mov    eax,switches
155:    83 e0 01                and    eax,0x1    // switches & 0x01
158:    74 06                   je     0x160

15a:    d9 ca                   fxch   st(2)
15c:    d9 e1                   fabs
15e:    d9 ca                   fxch   st(2)

160:    90                      nop

161:    dd 46 9c                fld    inv_zc     // inv_zc; t.z; t.y; t.x; fold;
164:    dc c1                   fadd   st(1),st   // inv_zc; t.z + inv_zc; t.y; t.x; fold;
166:    dd d8                   fstp   st(0)      // t.z + inv_zc; t.y; t.x; fold;

168:    90                      nop

169:    dd 46 a4                fld    inv_yc     // inv_yc; t.z + inv_zc; t.y; t.x; fold;
16c:    dc c2                   fadd   st(2),st   // inv_yc; t.z + inv_zc; t.y + inv_yc; t.x; fold;
16e:    dd d8                   fstp   st(0)      // t.z + inv_zc; t.y + inv_yc; t.x; fold;

170:    90                      nop

171:    dd 46 ac                fld    inv_xc     // inv_xc; t.z + inv_zc; t.y + inv_yc; t.x; fold;
174:    dc c3                   fadd   st(3),st   // inv_xc; t.z + inv_zc; t.y + inv_yc; t.x + inv_xc; fold;
176:    dd d8                   fstp   st(0)      // t.z + inv_zc; t.y + inv_yc; t.x + inv_xc; fold;
178:    d9 c0                   fld    st(0)      // t.z + inv_zc; t.z + inv_zc; t.y + inv_yc; t.x + inv_xc; fold;
17a:    d8 c8                   fmul   st,st(0)   // sqr(t.z + inv_zc); t.z + inv_zc; t.y + inv_yc; t.x + inv_xc; fold;
17c:    d9 c2                   fld    st(2)      // t.y + inv_yc; sqr(t.z + inv_zc); t.z + inv_zc; t.y + inv_yc; t.x + inv_xc; fold;
17e:    d8 c8                   fmul   st,st(0)   // sqr(t.y + inv_yc); sqr(t.z + inv_zc); t.z + inv_zc; t.y + inv_yc; t.x + inv_xc; fold;
180:    de c1                   faddp  st(1),st   // sqr(t.z + inv_zc) + sqr(t.y + inv_yc); t.z + inv_zc; t.y + inv_yc; t.x + inv_xc; fold;
182:    d9 c3                   fld    st(3)      // t.x + inv_xc; sqr(t.z + inv_zc) + sqr(t.y + inv_yc); t.z + inv_zc; t.y + inv_yc; t.x + inv_xc; fold;
184:    d8 c8                   fmul   st,st(0)   // sqr(t.x + inv_xc); sqr(t.z + inv_zc) + sqr(t.y + inv_yc); t.z + inv_zc; t.y + inv_yc; t.x + inv_xc; fold;
186:    de c1                   faddp  st(1),st   // sqr(t.z + inv_zc) + sqr(t.y + inv_yc) + sqr(t.x + inv_xc); t.z + inv_zc; t.y + inv_yc; t.x + inv_xc; fold;
188:    dc 4e 94                fmul   inv_radius // (sqr(t.z + inv_zc) + sqr(t.y + inv_yc) + sqr(t.x + inv_xc)) * inv_radius; t.z + inv_zc; t.y + inv_yc; t.x + inv_xc; fold;

18b:    90                      nop

18c:    dc 56 e0                fcom   sqr(minr)  // dot(t + i,t + i) * inv_radius; (t + i).z; (t + i).y; (t + i).x; fold;
18f:    df e0                   fnstsw ax
191:    d0 ec                   shr    ah,1
193:    73 07                   jae    0x19c

195:    dd d8                   fstp   st(0)      // (t + i).z; (t + i).y; (t + i).x; fold;
197:    dd 46 e8                fld    scale/sqr(minr)  // scale / sqr(minr); (t + i).z; (t + i).y; (t + i).x; fold;
19a:    eb 16                   jmp    0x1b2

19c:    d9 e8                   fld1              // 1; dot(t + i,t + i) * inv_radius; (t + i).z; (t + i).y; (t + i).x; fold;
19e:    d8 d1                   fcom   st(1)
1a0:    df e0                   fnstsw ax
1a2:    d0 ec                   shr    ah,1
1a4:    72 07                   jb     0x1ad

1a6:    dd d8                   fstp   st(0)      // dot(t + i,t + i) * inv_radius; (t + i).z; (t + i).y; (t + i).x; fold;
1a8:    dc 7e f0                fdivr  scale      // dot(t + i,t + i) * inv_radius / scale; (t + i).z; (t + i).y; (t + i).x; fold;
1ab:    eb 05                   jmp    0x1b2

1ad:    de d9                   fcompp            // (t + i).z; (t + i).y; (t + i).x; fold;
1af:    dd 46 f0                fld    scale      // scale; (t + i).z; (t + i).y; (t + i).x; fold;

v.w *= scale

1b2:    dd 41 08                fld    v.w        // v.w; scale; (t + i).z; (t + i).y; (t + i).x; fold;
1b5:    d8 c9                   fmul   st,st(1)   // v.w * scale; scale; (t + i).z; (t + i).y; (t + i).x; fold;
1b7:    dd 59 08                fstp   v.w        // v.w *= scale, scale; (t + i).z; (t + i).y; (t + i).x; fold;

p = (t + i - inv_c) * scale

1ba:    dc cb                   fmul   st(3),st   // scale; (t + i).z; (t + i).y; (t + i).x * scale; fold;
1bc:    dc ca                   fmul   st(2),st   // scale; (t + i).z; (t + i).y * scale; (t + i).x * scale; fold;
1be:    de c9                   fmulp  st(1),st   // (t + i).z * scale; (t + i).y * scale; (t + i).x * scale; fold;

1c0:    90                      nop

1c1:    dd 46 9c                fld    inv_zc     // inv_zc; (t + i).z * scale; (t + i).y * scale; (t + i).x * scale; fold;
1c4:    dc 4e f0                fmul   scale      // inv_zc * scale; (t + i).z * scale; (t + i).y * scale; (t + i).x * scale; fold;
1c7:    dc e9                   fsub   st(1),st   // inv_zc * scale; (t + i).z * scale - inv_zc * scale; (t + i).y * scale; (t + i).x * scale; fold;
1c9:    dd d8                   fstp   st(0)      // (t + i).z * scale - inv_zc * scale; (t + i).y * scale; (t + i).x * scale; fold;

1cb:    90                      nop

1cc:    dd 46 a4                fld    inv_yc     // inv_yc; (t + i).z * scale - inv_zc * scale; (t + i).y * scale; (t + i).x * scale; fold;
1cf:    dc 4e f0                fmul   scale      // inv_yc * scale; (t + i).z * scale - inv_zc * scale; (t + i).y * scale; (t + i).x * scale; fold;
1d2:    dc ea                   fsub   st(2),st   // inv_yc * scale; (t + i).z * scale - inv_zc * scale; (t + i).y * scale - inv_yc * scale; (t + i).x * scale; fold;
1d4:    dd d8                   fstp   st(0)      // (t + i).z * scale - inv_zc * scale; (t + i).y * scale - inv_yc * scale; (t + i).x * scale; fold;

1d6:    90                      nop

1d7:    dd 46 ac                fld    inv_xc     // inv_xc; (t + i).z * scale - inv_zc * scale; (t + i).y * scale - inv_yc * scale; (t + i).x * scale; fold;
1da:    dc 4e f0                fmul   scale      // inv_xc * scale; (t + i).z * scale - inv_zc * scale; (t + i).y * scale - inv_yc * scale; (t + i).x * scale; fold;
1dd:    dc eb                   fsub   st(3),st   // inv_xc * scale; (t + i).z * scale - inv_zc * scale; (t + i).y * scale - inv_yc * scale; (t + i).x * scale - inv_xc * scale; fold;
1df:    dd d8                   fstp   st(0)      // (t + i).z * scale - inv_zc * scale; (t + i).y * scale - inv_yc * scale; (t + i).x * scale - inv_xc * scale; fold;

1e1:    90                      nop

1e2:    d9 ca                   fxch   st(2)      // (t + i).x * scale - inv_xc * scale; (t + i).y * scale - inv_yc * scale; (t + i).z * scale - inv_zc * scale; fold;

housekeeping:

p.x = (t + i).x * scale - inv_xc * scale
p.y = (t + i).y * scale - inv_yc * scale
p.z = (t + i).z * scale - inv_zc * scale
fold

the rest is: v = rot * p + c

1e4:    d9 c0                   fld    st(0)      // p.x; p.y; p.z; fold;
1e6:    d8 4e bc                fmul   rot.z.x    // p.x * rot.z.x; p.x; p.y; p.z; fold;
1e9:    d9 c2                   fld    st(2)      // p.y; p.x * rot.z.x; p.x; p.y; p.z; fold;
1eb:    d8 4e b8                fmul   rot.z.y    // p.y * rot.z.y; p.x * rot.z.x; p.x; p.y; p.z; fold;
1ee:    de c1                   faddp  st(1),st   // p.x * rot.z.x + p.y * rot.z.y; p.x; p.y; p.z; fold;
1f0:    d9 c3                   fld    st(3)      // p.z; p.x * rot.z.x + p.y * rot.z.y; p.x; p.y; p.z; fold;
1f2:    d8 4e b4                fmul   rot.z.z    // p.z * rot.z.z; p.x * rot.z.x + p.y * rot.z.y; p.x; p.y; p.z; fold;
1f5:    de c1                   faddp  st(1),st   // dot(p,rot.z); p.x; p.y; p.z; fold;
1f7:    d9 c1                   fld    st(1)      // p.x; dot(p,rot.z); p.x; p.y; p.z; fold;
1f9:    d8 4e d4                fmul   rot.x.x    // p.x * rot.x.x; dot(p,rot.z); p.x; p.y; p.z; fold;
1fc:    d9 c3                   fld    st(3)      // p.y; p.x * rot.x.x; dot(p,rot.z); p.x; p.y; p.z; fold;
1fe:    d8 4e d0                fmul   rot.x.y    // p.y * rot.x.y; p.x * rot.x.x; dot(p,rot.z); p.x; p.y; p.z; fold;
201:    de c1                   faddp  st(1),st   // p.x * rot.x.x + p.y * rot.x.y; dot(p,rot.z); p.x; p.y; p.z; fold;
203:    d9 c4                   fld    st(4)      // p.z; p.x * rot.x.x + p.y * rot.x.y; dot(p,rot.z); p.x; p.y; p.z; fold;
205:    d8 4e cc                fmul   rot.x.z    // p.z * rot.x.z; p.x * rot.x.x + p.y * rot.x.y; dot(p,rot.z); p.x; p.y; p.z; fold;
208:    de c1                   faddp  st(1),st   // dot(p,rot.x); dot(p,rot.z); p.x; p.y; p.z; fold;
20a:    d9 ca                   fxch   st(2)      // p.x; dot(p,rot.z); dot(p,rot.x); p.y; p.z; fold;
20c:    d8 4e c8                fmul   rot.y.x    // p.x * rot.y.x; dot(p,rot.z); dot(p,rot.x); p.y; p.z; fold;
20f:    d9 cb                   fxch   st(3)      // p.y; dot(p,rot.z); dot(p,rot.x); p.x * rot.y.x; p.z; fold;
211:    d8 4e c4                fmul   rot.y.y    // p.y * rot.y.y; dot(p,rot.z); dot(p,rot.x); p.x * rot.y.x; p.z; fold;
214:    de c3                   faddp  st(3),st   // dot(p,rot.z); dot(p,rot.x); p.x * rot.y.x + p.y * rot.y.y; p.z; fold;
216:    d9 cb                   fxch   st(3)      // p.z; dot(p,rot.x); p.x * rot.y.x + p.y * rot.y.y; dot(p,rot.z); fold;
218:    d8 4e c0                fmul   rot.y.z    // p.z * rot.y.z; dot(p,rot.x); p.x * rot.y.x + p.y * rot.y.y; dot(p,rot.z); fold;
21b:    de c2                   faddp  st(2),st   // dot(p,rot.x); dot(p.rot.y); dot(p,rot.z); fold;
21d:    d9 ca                   fxch   st(2)      // dot(p,rot.z); dot(p.rot.y); dot(p,rot.x); fold;
21f:    8b 75 08                mov    esi,DWORD PTR [ebp+0x8]
222:    dc 46 28                fadd   c.z        // dot(p,rot.z) + c.z; dot(p.rot.y); dot(p,rot.x); fold;
225:    dd 19                   fstp   v.z        // v.z = dot(p,rot.z) + c.z, dot(p.rot.y); dot(p,rot.x); fold;
227:    dc 46 20                fadd   c.y        // dot(p.rot.y) + c.y; dot(p,rot.x); fold;
22a:    dd 1a                   fstp   v.y        // v.y = dot(p.rot.y) + c.y, dot(p,rot.x); fold;
22c:    dc 46 18                fadd   c.x        // dot(p,rot.x) + c.x; fold;
22f:    dd 1b                   fstp   v.x        // v.x = dot(p,rot.x) + c.x, fold;
231:    dd d8                   fstp   st(0)
233:    83 c4 30                add    esp,0x30
236:    89 d8                   mov    eax,ebx
238:    5b                      pop    ebx
239:    5e                      pop    esi
23a:    5d                      pop    ebp
23b:    c2 08 00                ret    0x8
```

## Code

```glsl

t0 = v.x * fx_sinxz - v.z * fx_cosxz;
t1 = v.x * fx_cosxy + v.z * fx_sinxy;
t2 = t0 * fx_cosxz + v.y * fx_sinxz;

t3 = t0 * fx_sinxz - v.y * fx_cosxz;
t4 = |t3 + fold| - |t3 - fold| - t3;
t5 = t4 * fx_sinxz - t2 * fx_cosxz;
t.x = t5 * fx_sinxy - t1 * fx_cosxy;

t6 = t0 * fy_cosxy - v.x * fy_sinxy;
t7 = |t6 + fold| - |t6 - fold| - t6;
t8 = t7 * fy_cosxy - t2 * fy_sinxy;
t.y = t8 * fy_cosyz - t1 * fy_sinyz;

t.z = |v.z + fold| - |v.z - fold| - v.z;

p = t * scale;
v = rot * p + c;
dr *= scale;
```