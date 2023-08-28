# `JCube3`

```
[OPTIONS]
.Version = 4
.DEscale = 0.2
.DEoption = 2
.RStop = 1024
.DReci2 Alpha = .41421356
.Double Cx1 = 0
.Double Cy1 = 1
.Double Cz1 = 1
.Double Cx2 = 1
.Double Cy2 = 1
.Double Cz2 = 1
.Double GScale (test) = 3
[CODE]
558BEC535689C38B75088B7630B8FFFFFF7F2141042142042143049083C4B0DD
46B0D9E8D8E9D9C9D9E0DD5C2418DC46F0DD1424DC4EE8DD542408D9E8DEF1DD
542410DC4C2418D9E8DEC1DD5C241890DD01DD02DD03D8D1DFE0D0EC7202D9C9
D8D2DFE0D0EC7302D9CAD9C9D8D2DFE0D0EC7202D9CA90D9C0DD442410DED9DF
E0D0EC765290D9C0DC442418D9C2DED9DFE0D0EC764190DD46E0DCE9DDD8DD46
D8DCEADDD8DD46D0DCEBDDD8DD442408DCCBDCCADCC9DD4108DEC9DD5908DD46
E0DCC1DDD8DD46D8DCC2DDD8DD46D0DCC3DDD890EB3E90DD46C8DCE9DDD8DD46
C0DCEADDD8DD46B8DCEBDDD8DD042490DCCBDCCADCC9DD4108DEC9DD5908DD46
C8DCC1DDD8DD46C0DCC2DDD8DD46B8DCC3DDD89090DD1BDD1ADD1989D883C450
5E5B5DC20800
[END]

A brute-force IFS formula, that tries to replicate the "Jerusalem Cube" fractal.
It is discontinue, but it's the best known approximation of the formula.
Warning; GScale parameter may help to obtain more effects but discontinuities become more evident!
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
d:  b8 ff ff ff 7f          mov    eax,0x7fffffff
12: 21 41 04                and    DWORD PTR [ecx+0x4],eax
15: 21 42 04                and    DWORD PTR [edx+0x4],eax
18: 21 43 04                and    DWORD PTR [ebx+0x4],eax
1b: 90                      nop
1c: 83 c4 b0                add    esp,0xffffffb0
1f: dd 46 b0                fld    QWORD PTR [esi-0x50]
22: d9 e8                   fld1
24: d8 e9                   fsubr  st,st(1)
26: d9 c9                   fxch   st(1)
28: d9 e0                   fchs
2a: dd 5c 24 18             fstp   QWORD PTR [esp+0x18]
2e: dc 46 f0                fadd   QWORD PTR [esi-0x10]
31: dd 14 24                fst    QWORD PTR [esp]
34: dc 4e e8                fmul   QWORD PTR [esi-0x18]
37: dd 54 24 08             fst    QWORD PTR [esp+0x8]
3b: d9 e8                   fld1
3d: de f1                   fdivrp st(1),st
3f: dd 54 24 10             fst    QWORD PTR [esp+0x10]
43: dc 4c 24 18             fmul   QWORD PTR [esp+0x18]
47: d9 e8                   fld1
49: de c1                   faddp  st(1),st
4b: dd 5c 24 18             fstp   QWORD PTR [esp+0x18]
4f: 90                      nop
50: dd 01                   fld    QWORD PTR [ecx]
52: dd 02                   fld    QWORD PTR [edx]
54: dd 03                   fld    QWORD PTR [ebx]
56: d8 d1                   fcom   st(1)
58: df e0                   fnstsw ax
5a: d0 ec                   shr    ah,1
5c: 72 02                   jb     0x60
5e: d9 c9                   fxch   st(1)
60: d8 d2                   fcom   st(2)
62: df e0                   fnstsw ax
64: d0 ec                   shr    ah,1
66: 73 02                   jae    0x6a
68: d9 ca                   fxch   st(2)
6a: d9 c9                   fxch   st(1)
6c: d8 d2                   fcom   st(2)
6e: df e0                   fnstsw ax
70: d0 ec                   shr    ah,1
72: 72 02                   jb     0x76
74: d9 ca                   fxch   st(2)
76: 90                      nop
77: d9 c0                   fld    st(0)
79: dd 44 24 10             fld    QWORD PTR [esp+0x10]
7d: de d9                   fcompp
7f: df e0                   fnstsw ax
81: d0 ec                   shr    ah,1
83: 76 52                   jbe    0xd7
85: 90                      nop
86: d9 c0                   fld    st(0)
88: dc 44 24 18             fadd   QWORD PTR [esp+0x18]
8c: d9 c2                   fld    st(2)
8e: de d9                   fcompp
90: df e0                   fnstsw ax
92: d0 ec                   shr    ah,1
94: 76 41                   jbe    0xd7
96: 90                      nop
97: dd 46 e0                fld    QWORD PTR [esi-0x20]
9a: dc e9                   fsub   st(1),st
9c: dd d8                   fstp   st(0)
9e: dd 46 d8                fld    QWORD PTR [esi-0x28]
a1: dc ea                   fsub   st(2),st
a3: dd d8                   fstp   st(0)
a5: dd 46 d0                fld    QWORD PTR [esi-0x30]
a8: dc eb                   fsub   st(3),st
aa: dd d8                   fstp   st(0)
ac: dd 44 24 08             fld    QWORD PTR [esp+0x8]
b0: dc cb                   fmul   st(3),st
b2: dc ca                   fmul   st(2),st
b4: dc c9                   fmul   st(1),st
b6: dd 41 08                fld    QWORD PTR [ecx+0x8]
b9: de c9                   fmulp  st(1),st
bb: dd 59 08                fstp   QWORD PTR [ecx+0x8]
be: dd 46 e0                fld    QWORD PTR [esi-0x20]
c1: dc c1                   fadd   st(1),st
c3: dd d8                   fstp   st(0)
c5: dd 46 d8                fld    QWORD PTR [esi-0x28]
c8: dc c2                   fadd   st(2),st
ca: dd d8                   fstp   st(0)
cc: dd 46 d0                fld    QWORD PTR [esi-0x30]
cf: dc c3                   fadd   st(3),st
d1: dd d8                   fstp   st(0)
d3: 90                      nop
d4: eb 3e                   jmp    0x114
d6: 90                      nop
d7: dd 46 c8                fld    QWORD PTR [esi-0x38]
da: dc e9                   fsub   st(1),st
dc: dd d8                   fstp   st(0)
de: dd 46 c0                fld    QWORD PTR [esi-0x40]
e1: dc ea                   fsub   st(2),st
e3: dd d8                   fstp   st(0)
e5: dd 46 b8                fld    QWORD PTR [esi-0x48]
e8: dc eb                   fsub   st(3),st
ea: dd d8                   fstp   st(0)
ec: dd 04 24                fld    QWORD PTR [esp]
ef: 90                      nop
f0: dc cb                   fmul   st(3),st
f2: dc ca                   fmul   st(2),st
f4: dc c9                   fmul   st(1),st
f6: dd 41 08                fld    QWORD PTR [ecx+0x8]
f9: de c9                   fmulp  st(1),st
fb: dd 59 08                fstp   QWORD PTR [ecx+0x8]
fe: dd 46 c8                fld    QWORD PTR [esi-0x38]
101:    dc c1                   fadd   st(1),st
103:    dd d8                   fstp   st(0)
105:    dd 46 c0                fld    QWORD PTR [esi-0x40]
108:    dc c2                   fadd   st(2),st
10a:    dd d8                   fstp   st(0)
10c:    dd 46 b8                fld    QWORD PTR [esi-0x48]
10f:    dc c3                   fadd   st(3),st
111:    dd d8                   fstp   st(0)
113:    90                      nop
114:    90                      nop
115:    dd 1b                   fstp   QWORD PTR [ebx]
117:    dd 1a                   fstp   QWORD PTR [edx]
119:    dd 19                   fstp   QWORD PTR [ecx]
11b:    89 d8                   mov    eax,ebx
11d:    83 c4 50                add    esp,0x50
120:    5e                      pop    esi
121:    5b                      pop    ebx
122:    5d                      pop    ebp
123:    c2 08 00                ret    0x8
```
