# `MengerIFS`

```
[OPTIONS]
.Version = 2
.DEscale = 0.2
.DEoption = 2
.RStop = 1024
.Double Scale = 3
.Double CScale X = 1
.Double CScale Y = 1
.Double CScale Z = 0.5
.3SingleAngles Rotation = 0
[CODE]
558BEC535689C38B75088B7630DD01D9E1DD02D9E1DD03D9E1D8D1DFE0D0EC73
02D9C9D8D2DFE0D0EC7302D9CAD9C9D8D2DFE0D0EC7302D9CAD9C1D84EBCD9C1
D84EB8DEC1D9C3D84EB4DEC1D9C2D84ED4D9C2D84ED0DEC1D9C4D84ECCDEC1D9
CBD84EC8D9CAD84EC4DEC2D9CBD84EC0DEC1DD46F0DD4108D8C9DD5908DCCADC
C9D9C0D9E8DEE9D9C0D9C0DC4EE8DEEDDC4EE0DEEBD8F1DC4ED8DCECD9CCD9E1
DEECDECBDD1ADD1BDD1989D85E5B5DC20800
[END]

Description:

A 3d menger version that hormonizes more in hybrids then 'menger3'.
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
f:  d9 e1                   fabs
11: dd 02                   fld    QWORD PTR [edx]
13: d9 e1                   fabs
15: dd 03                   fld    QWORD PTR [ebx]
17: d9 e1                   fabs
19: d8 d1                   fcom   st(1)
1b: df e0                   fnstsw ax
1d: d0 ec                   shr    ah,1
1f: 73 02                   jae    0x23
21: d9 c9                   fxch   st(1)
23: d8 d2                   fcom   st(2)
25: df e0                   fnstsw ax
27: d0 ec                   shr    ah,1
29: 73 02                   jae    0x2d
2b: d9 ca                   fxch   st(2)
2d: d9 c9                   fxch   st(1)
2f: d8 d2                   fcom   st(2)
31: df e0                   fnstsw ax
33: d0 ec                   shr    ah,1
35: 73 02                   jae    0x39
37: d9 ca                   fxch   st(2)
39: d9 c1                   fld    st(1)
3b: d8 4e bc                fmul   DWORD PTR [esi-0x44]
3e: d9 c1                   fld    st(1)
40: d8 4e b8                fmul   DWORD PTR [esi-0x48]
43: de c1                   faddp  st(1),st
45: d9 c3                   fld    st(3)
47: d8 4e b4                fmul   DWORD PTR [esi-0x4c]
4a: de c1                   faddp  st(1),st
4c: d9 c2                   fld    st(2)
4e: d8 4e d4                fmul   DWORD PTR [esi-0x2c]
51: d9 c2                   fld    st(2)
53: d8 4e d0                fmul   DWORD PTR [esi-0x30]
56: de c1                   faddp  st(1),st
58: d9 c4                   fld    st(4)
5a: d8 4e cc                fmul   DWORD PTR [esi-0x34]
5d: de c1                   faddp  st(1),st
5f: d9 cb                   fxch   st(3)
61: d8 4e c8                fmul   DWORD PTR [esi-0x38]
64: d9 ca                   fxch   st(2)
66: d8 4e c4                fmul   DWORD PTR [esi-0x3c]
69: de c2                   faddp  st(2),st
6b: d9 cb                   fxch   st(3)
6d: d8 4e c0                fmul   DWORD PTR [esi-0x40]
70: de c1                   faddp  st(1),st
72: dd 46 f0                fld    QWORD PTR [esi-0x10]
75: dd 41 08                fld    QWORD PTR [ecx+0x8]
78: d8 c9                   fmul   st,st(1)
7a: dd 59 08                fstp   QWORD PTR [ecx+0x8]
7d: dc ca                   fmul   st(2),st
7f: dc c9                   fmul   st(1),st
81: d9 c0                   fld    st(0)
83: d9 e8                   fld1
85: de e9                   fsubp  st(1),st
87: d9 c0                   fld    st(0)
89: d9 c0                   fld    st(0)
8b: dc 4e e8                fmul   QWORD PTR [esi-0x18]
8e: de ed                   fsubp  st(5),st
90: dc 4e e0                fmul   QWORD PTR [esi-0x20]
93: de eb                   fsubp  st(3),st
95: d8 f1                   fdiv   st,st(1)
97: dc 4e d8                fmul   QWORD PTR [esi-0x28]
9a: dc ec                   fsub   st(4),st
9c: d9 cc                   fxch   st(4)
9e: d9 e1                   fabs
a0: de ec                   fsubp  st(4),st
a2: de cb                   fmulp  st(3),st
a4: dd 1a                   fstp   QWORD PTR [edx]
a6: dd 1b                   fstp   QWORD PTR [ebx]
a8: dd 19                   fstp   QWORD PTR [ecx]
aa: 89 d8                   mov    eax,ebx
ac: 5e                      pop    esi
ad: 5b                      pop    ebx
ae: 5d                      pop    ebp
af: c2 08 00                ret    0x8
```
