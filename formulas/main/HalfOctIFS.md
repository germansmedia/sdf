# `HalfOctIFS`

```
[OPTIONS]
.Version = 2
.DEscale = 0.2
.DEoption = 2
.RStop = 1024
.Double Scale = 2
.Double CScale X = 0
.Double CScale Y = 0
.Double CScale Z = 1
.3SingleAngles Rotation1 = 0
.3SingleAngles Rotation2 = 0
[CODE]
558BEC535689C38B75088B7630DD01DD02DD03D9C0D84EBCD9C2D84EB8DEC1D9
C3D84EB4DEC1D9C1D84ED4D9C3D84ED0DEC1D9C4D84ECCDEC1D9CAD84EC8D9CB
D84EC4DEC3D9CBD84EC0DEC2D8D1DFE0D0EC7302D9C9D9E0D8D1DFE0D0EC7202
D9C9D9E0D8D2DFE0D0EC7202D9CAD9E0D9C9D9C1D84E98D9C1D84E94DEC1D9C3
D84E90DEC1D9C2D84EB0D9C2D84EACDEC1D9C4D84EA8DEC1D9CBD84EA4D9CAD8
4EA0DEC2D9CBD84E9CDEC1DD46F0DD4108D8C9DD5908DCCBDCCADCC9D9E8DEE9
D9C0D9C0DC4EE8DEECDC4EE0DEEADC4ED8DEEBDD1ADD1BDD1989D85E5B5DC208
00
[END]

A 3D half-octahedron attempt; same as normal one but the third fold is dropped.
Nice shapes can be obtained with this variation.

Luca GN 2011
```

## Disassembly

```
0:  55                      push   ebp
1:  8b ec                   mov    ebp,esp
3:  53                      push   ebx
4:  56                      push   esi
5:  89 c3                   mov    ebx,eax
7:  8b 75 08                mov    esi,DWORD PTR [ebp+0x8]
a:  8b 76 30                mov    esi,DWORD PTR [esi+0x30]
d:  dd 01                   fld    QWORD PTR [ecx]
f:  dd 02                   fld    QWORD PTR [edx]
11: dd 03                   fld    QWORD PTR [ebx]
13: d9 c0                   fld    st(0)
15: d8 4e bc                fmul   DWORD PTR [esi-0x44]
18: d9 c2                   fld    st(2)
1a: d8 4e b8                fmul   DWORD PTR [esi-0x48]
1d: de c1                   faddp  st(1),st
1f: d9 c3                   fld    st(3)
21: d8 4e b4                fmul   DWORD PTR [esi-0x4c]
24: de c1                   faddp  st(1),st
26: d9 c1                   fld    st(1)
28: d8 4e d4                fmul   DWORD PTR [esi-0x2c]
2b: d9 c3                   fld    st(3)
2d: d8 4e d0                fmul   DWORD PTR [esi-0x30]
30: de c1                   faddp  st(1),st
32: d9 c4                   fld    st(4)
34: d8 4e cc                fmul   DWORD PTR [esi-0x34]
37: de c1                   faddp  st(1),st
39: d9 ca                   fxch   st(2)
3b: d8 4e c8                fmul   DWORD PTR [esi-0x38]
3e: d9 cb                   fxch   st(3)
40: d8 4e c4                fmul   DWORD PTR [esi-0x3c]
43: de c3                   faddp  st(3),st
45: d9 cb                   fxch   st(3)
47: d8 4e c0                fmul   DWORD PTR [esi-0x40]
4a: de c2                   faddp  st(2),st
4c: d8 d1                   fcom   st(1)
4e: df e0                   fnstsw ax
50: d0 ec                   shr    ah,1
52: 73 02                   jae    0x56
54: d9 c9                   fxch   st(1)
56: d9 e0                   fchs
58: d8 d1                   fcom   st(1)
5a: df e0                   fnstsw ax
5c: d0 ec                   shr    ah,1
5e: 72 02                   jb     0x62
60: d9 c9                   fxch   st(1)
62: d9 e0                   fchs
64: d8 d2                   fcom   st(2)
66: df e0                   fnstsw ax
68: d0 ec                   shr    ah,1
6a: 72 02                   jb     0x6e
6c: d9 ca                   fxch   st(2)
6e: d9 e0                   fchs
70: d9 c9                   fxch   st(1)
72: d9 c1                   fld    st(1)
74: d8 4e 98                fmul   DWORD PTR [esi-0x68]
77: d9 c1                   fld    st(1)
79: d8 4e 94                fmul   DWORD PTR [esi-0x6c]
7c: de c1                   faddp  st(1),st
7e: d9 c3                   fld    st(3)
80: d8 4e 90                fmul   DWORD PTR [esi-0x70]
83: de c1                   faddp  st(1),st
85: d9 c2                   fld    st(2)
87: d8 4e b0                fmul   DWORD PTR [esi-0x50]
8a: d9 c2                   fld    st(2)
8c: d8 4e ac                fmul   DWORD PTR [esi-0x54]
8f: de c1                   faddp  st(1),st
91: d9 c4                   fld    st(4)
93: d8 4e a8                fmul   DWORD PTR [esi-0x58]
96: de c1                   faddp  st(1),st
98: d9 cb                   fxch   st(3)
9a: d8 4e a4                fmul   DWORD PTR [esi-0x5c]
9d: d9 ca                   fxch   st(2)
9f: d8 4e a0                fmul   DWORD PTR [esi-0x60]
a2: de c2                   faddp  st(2),st
a4: d9 cb                   fxch   st(3)
a6: d8 4e 9c                fmul   DWORD PTR [esi-0x64]
a9: de c1                   faddp  st(1),st
ab: dd 46 f0                fld    QWORD PTR [esi-0x10]
ae: dd 41 08                fld    QWORD PTR [ecx+0x8]
b1: d8 c9                   fmul   st,st(1)
b3: dd 59 08                fstp   QWORD PTR [ecx+0x8]
b6: dc cb                   fmul   st(3),st
b8: dc ca                   fmul   st(2),st
ba: dc c9                   fmul   st(1),st
bc: d9 e8                   fld1
be: de e9                   fsubp  st(1),st
c0: d9 c0                   fld    st(0)
c2: d9 c0                   fld    st(0)
c4: dc 4e e8                fmul   QWORD PTR [esi-0x18]
c7: de ec                   fsubp  st(4),st
c9: dc 4e e0                fmul   QWORD PTR [esi-0x20]
cc: de ea                   fsubp  st(2),st
ce: dc 4e d8                fmul   QWORD PTR [esi-0x28]
d1: de eb                   fsubp  st(3),st
d3: dd 1a                   fstp   QWORD PTR [edx]
d5: dd 1b                   fstp   QWORD PTR [ebx]
d7: dd 19                   fstp   QWORD PTR [ecx]
d9: 89 d8                   mov    eax,ebx
db: 5e                      pop    esi
dc: 5b                      pop    ebx
dd: 5d                      pop    ebp
de: c2 08 00                ret    0x8
```
