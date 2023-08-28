# `ABoxSphereOffset4d`

```
[OPTIONS]
.Version = 5
.DEscale = 0.2
.DEoption = 5
.RStop = 1024
.SIpower = 2
.Double Scale = 2
.Boxscale Min R = 0.5
.Double Fold = 1
.Double Sphere offset X = 0
.Double Sphere offset Y = 0
.Double Sphere offset Z = 0
.Double Sphere offset W = 0
.Double W add = 0
[CODE]
558BEC56578B7D088B773081C780000000DD46D8DD00D9C0D8E2D9E1D8C1D9C9
D8C2D9E1DEE1DC66D0DD02D9C0D8E3D9E1D8C1D9C9D8C3D9E1DEE1DC66C8DD01
D9C0D8E4D9E1D8C1D9C9D8C4D9E1DEE1DC66C0DD4108D9C0D8E5D9E1D8C1D9C9
D8C5D9E1DEE1DC66B8D9CCDDD8D9C0D8C9D9C2D8CBDEC1D9C3D8CCDEC1D9C4D8
CDDEC1DC56E0DFE0D0EC7307DDD8DD46E8EB13D9E8D8D9DFE0DD46F0D0EC7202
D8F1D9C9DDD8DD4768D8C9DD5F68DCCCDCCBDCCADEC9DC47A8DC46C0DD19DC47
A0DC46C8DD1ADC4798DC46D0DD5AF8DC46B0DC46B8DC8748FFFFFFDD59085F5E
5DC20800
[END]

Description:

A 4d "Amazing Box" (invented by TGlad) aka Mandbox variation,
having an offset for the sphere folding:

x = abs(x+Fold) - abs(x-Fold) - x - Sphere_offset_X
y = abs(y+Fold) - abs(y-Fold) - y - Sphere_offset_Y
z = abs(z+Fold) - abs(z-Fold) - z - Sphere_offset_Z
w = abs(w+Fold) - abs(w-Fold) - w - Sphere_offset_W
rr = x*x + y*y + z*z + w*w
if rr < sqr(Min_R) then m = Scale/sqr(Min_R) else
if rr < 1 then m = Scale/rr else m = Scale
x = x * m + Sphere_offset_X + Cx
y = y * m + Sphere_offset_Y + Cy
z = z * m + Sphere_offset_Z + Cz
w = w * m + Sphere_offset_W + W_add + Cw
```

## Disassembly

```
0:  55                      push   ebp
1:  8b ec                   mov    ebp,esp
3:  56                      push   esi
4:  57                      push   edi
5:  8b 7d 08                mov    edi,DWORD PTR [ebp+0x8]
8:  8b 77 30                mov    esi,DWORD PTR [edi+0x30]
b:  81 c7 80 00 00 00       add    edi,0x80
11: dd 46 d8                fld    QWORD PTR [esi-0x28]
14: dd 00                   fld    QWORD PTR [eax]
16: d9 c0                   fld    st(0)
18: d8 e2                   fsub   st,st(2)
1a: d9 e1                   fabs
1c: d8 c1                   fadd   st,st(1)
1e: d9 c9                   fxch   st(1)
20: d8 c2                   fadd   st,st(2)
22: d9 e1                   fabs
24: de e1                   fsubrp st(1),st
26: dc 66 d0                fsub   QWORD PTR [esi-0x30]
29: dd 02                   fld    QWORD PTR [edx]
2b: d9 c0                   fld    st(0)
2d: d8 e3                   fsub   st,st(3)
2f: d9 e1                   fabs
31: d8 c1                   fadd   st,st(1)
33: d9 c9                   fxch   st(1)
35: d8 c3                   fadd   st,st(3)
37: d9 e1                   fabs
39: de e1                   fsubrp st(1),st
3b: dc 66 c8                fsub   QWORD PTR [esi-0x38]
3e: dd 01                   fld    QWORD PTR [ecx]
40: d9 c0                   fld    st(0)
42: d8 e4                   fsub   st,st(4)
44: d9 e1                   fabs
46: d8 c1                   fadd   st,st(1)
48: d9 c9                   fxch   st(1)
4a: d8 c4                   fadd   st,st(4)
4c: d9 e1                   fabs
4e: de e1                   fsubrp st(1),st
50: dc 66 c0                fsub   QWORD PTR [esi-0x40]
53: dd 41 08                fld    QWORD PTR [ecx+0x8]
56: d9 c0                   fld    st(0)
58: d8 e5                   fsub   st,st(5)
5a: d9 e1                   fabs
5c: d8 c1                   fadd   st,st(1)
5e: d9 c9                   fxch   st(1)
60: d8 c5                   fadd   st,st(5)
62: d9 e1                   fabs
64: de e1                   fsubrp st(1),st
66: dc 66 b8                fsub   QWORD PTR [esi-0x48]
69: d9 cc                   fxch   st(4)
6b: dd d8                   fstp   st(0)
6d: d9 c0                   fld    st(0)
6f: d8 c9                   fmul   st,st(1)
71: d9 c2                   fld    st(2)
73: d8 cb                   fmul   st,st(3)
75: de c1                   faddp  st(1),st
77: d9 c3                   fld    st(3)
79: d8 cc                   fmul   st,st(4)
7b: de c1                   faddp  st(1),st
7d: d9 c4                   fld    st(4)
7f: d8 cd                   fmul   st,st(5)
81: de c1                   faddp  st(1),st
83: dc 56 e0                fcom   QWORD PTR [esi-0x20]
86: df e0                   fnstsw ax
88: d0 ec                   shr    ah,1
8a: 73 07                   jae    0x93
8c: dd d8                   fstp   st(0)
8e: dd 46 e8                fld    QWORD PTR [esi-0x18]
91: eb 13                   jmp    0xa6
93: d9 e8                   fld1
95: d8 d9                   fcomp  st(1)
97: df e0                   fnstsw ax
99: dd 46 f0                fld    QWORD PTR [esi-0x10]
9c: d0 ec                   shr    ah,1
9e: 72 02                   jb     0xa2
a0: d8 f1                   fdiv   st,st(1)
a2: d9 c9                   fxch   st(1)
a4: dd d8                   fstp   st(0)
a6: dd 47 68                fld    QWORD PTR [edi+0x68]
a9: d8 c9                   fmul   st,st(1)
ab: dd 5f 68                fstp   QWORD PTR [edi+0x68]
ae: dc cc                   fmul   st(4),st
b0: dc cb                   fmul   st(3),st
b2: dc ca                   fmul   st(2),st
b4: de c9                   fmulp  st(1),st
b6: dc 47 a8                fadd   QWORD PTR [edi-0x58]
b9: dc 46 c0                fadd   QWORD PTR [esi-0x40]
bc: dd 19                   fstp   QWORD PTR [ecx]
be: dc 47 a0                fadd   QWORD PTR [edi-0x60]
c1: dc 46 c8                fadd   QWORD PTR [esi-0x38]
c4: dd 1a                   fstp   QWORD PTR [edx]
c6: dc 47 98                fadd   QWORD PTR [edi-0x68]
c9: dc 46 d0                fadd   QWORD PTR [esi-0x30]
cc: dd 5a f8                fstp   QWORD PTR [edx-0x8]
cf: dc 46 b0                fadd   QWORD PTR [esi-0x50]
d2: dc 46 b8                fadd   QWORD PTR [esi-0x48]
d5: dc 87 48 ff ff ff       fadd   QWORD PTR [edi-0xb8]
db: dd 59 08                fstp   QWORD PTR [ecx+0x8]
de: 5f                      pop    edi
df: 5e                      pop    esi
e0: 5d                      pop    ebp
e1: c2 08 00                ret    0x8
```
