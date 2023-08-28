# `MixPinski4ex`

```
[OPTIONS]
.Version = 6
.DEscale = 0.2
.ADEscale = 2
.DEoption = 6
.RStop = 1024
.SIpower = 2
.Double Scale = 2
.Double CScale X = 1
.Double CScale Y = 1
.Double CScale Z = 0.5
.Double CScale W = 0.5
.6SingleAngles Rotation = 0
.Double Rot X-off = 0.5
.Double Rot Y-off = 0.5
.Double Rot Z-off = 0.5
.Double Rot W-off = 0.5
[CODE]
558BEC53565789C38B75088B7E30DD41
08DD01DD02DD0390D9E0D8D1DFE0D0EC
7202D9C9D8D2DFE0D0EC7202D9CAD9E0
D9C9D9E0D8D2DFE0D0EC7202D9CAD9E0
D9C990D9E0D8D3DFE0D0EC7202D9CBD9
E0D9C9D9E0D8D3DFE0D0EC7202D9CBD9
E0D9C9D9CAD9E0D8D3DFE0D0EC7202D9
CBD9E0D9CADD4780DEC1DD4788DEC2DD
8778FFFFFFDEC3DD8770FFFFFFDEC490
83EF28D9C0D84FF4D9C2D84FF0DEC1D9
C3D84FECDEC1D9C4D84FE8DEC1DD1BD9
C0D84FE4D9C2D84FE0DEC1D9C3D84FDC
DEC1D9C4D84FD8DEC1DD1AD9C0D84FD4
D9C2D84FD0DEC1D9C3D84FCCDEC1D9C4
D84FC8DEC1DD19D84FC4D9C9D84FC0DE
C1D9C9D84FBCDEC1D9C9D84FB8DEC183
C728DD8770FFFFFFDEE9DD47F0DCC9D9
E8DEE9DC4FD0DEE9DD5908DD01DCA778
FFFFFFDD02DC6788DD03DC678090DD47
F0DD86E8000000D8C9DD9EE800000090
90DCCADCC9D9C0D9E8DEE9D9C0D9C0DC
4FE8DEEDDC4FE0DEEBD8F1DC4FD8DCEC
D9CCD9E1DEECDECBDD1ADD1BDD199090
9089D85F5E5B5DC20800
[END]

Description:

NOTES: See Sierpinski4ex and Menger4ex for any explanation.

LUCA GN 2011
```

## Disassembly

```
0:  55                      push   ebp
1:  8b ec                   mov    ebp,esp
3:  53                      push   ebx
4:  56                      push   esi
5:  57                      push   edi
6:  89 c3                   mov    ebx,eax
8:  8b 75 08                mov    esi,DWORD PTR [ebp+0x8]
b:  8b 7e 30                mov    edi,DWORD PTR [esi+0x30]
e:  dd 41 08                fld    QWORD PTR [ecx+0x8]
11: dd 01                   fld    QWORD PTR [ecx]
13: dd 02                   fld    QWORD PTR [edx]
15: dd 03                   fld    QWORD PTR [ebx]
17: 90                      nop
18: d9 e0                   fchs
1a: d8 d1                   fcom   st(1)
1c: df e0                   fnstsw ax
1e: d0 ec                   shr    ah,1
20: 72 02                   jb     0x24
22: d9 c9                   fxch   st(1)
24: d8 d2                   fcom   st(2)
26: df e0                   fnstsw ax
28: d0 ec                   shr    ah,1
2a: 72 02                   jb     0x2e
2c: d9 ca                   fxch   st(2)
2e: d9 e0                   fchs
30: d9 c9                   fxch   st(1)
32: d9 e0                   fchs
34: d8 d2                   fcom   st(2)
36: df e0                   fnstsw ax
38: d0 ec                   shr    ah,1
3a: 72 02                   jb     0x3e
3c: d9 ca                   fxch   st(2)
3e: d9 e0                   fchs
40: d9 c9                   fxch   st(1)
42: 90                      nop
43: d9 e0                   fchs
45: d8 d3                   fcom   st(3)
47: df e0                   fnstsw ax
49: d0 ec                   shr    ah,1
4b: 72 02                   jb     0x4f
4d: d9 cb                   fxch   st(3)
4f: d9 e0                   fchs
51: d9 c9                   fxch   st(1)
53: d9 e0                   fchs
55: d8 d3                   fcom   st(3)
57: df e0                   fnstsw ax
59: d0 ec                   shr    ah,1
5b: 72 02                   jb     0x5f
5d: d9 cb                   fxch   st(3)
5f: d9 e0                   fchs
61: d9 c9                   fxch   st(1)
63: d9 ca                   fxch   st(2)
65: d9 e0                   fchs
67: d8 d3                   fcom   st(3)
69: df e0                   fnstsw ax
6b: d0 ec                   shr    ah,1
6d: 72 02                   jb     0x71
6f: d9 cb                   fxch   st(3)
71: d9 e0                   fchs
73: d9 ca                   fxch   st(2)
75: dd 47 80                fld    QWORD PTR [edi-0x80]
78: de c1                   faddp  st(1),st
7a: dd 47 88                fld    QWORD PTR [edi-0x78]
7d: de c2                   faddp  st(2),st
7f: dd 87 78 ff ff ff       fld    QWORD PTR [edi-0x88]
85: de c3                   faddp  st(3),st
87: dd 87 70 ff ff ff       fld    QWORD PTR [edi-0x90]
8d: de c4                   faddp  st(4),st
8f: 90                      nop
90: 83 ef 28                sub    edi,0x28
93: d9 c0                   fld    st(0)
95: d8 4f f4                fmul   DWORD PTR [edi-0xc]
98: d9 c2                   fld    st(2)
9a: d8 4f f0                fmul   DWORD PTR [edi-0x10]
9d: de c1                   faddp  st(1),st
9f: d9 c3                   fld    st(3)
a1: d8 4f ec                fmul   DWORD PTR [edi-0x14]
a4: de c1                   faddp  st(1),st
a6: d9 c4                   fld    st(4)
a8: d8 4f e8                fmul   DWORD PTR [edi-0x18]
ab: de c1                   faddp  st(1),st
ad: dd 1b                   fstp   QWORD PTR [ebx]
af: d9 c0                   fld    st(0)
b1: d8 4f e4                fmul   DWORD PTR [edi-0x1c]
b4: d9 c2                   fld    st(2)
b6: d8 4f e0                fmul   DWORD PTR [edi-0x20]
b9: de c1                   faddp  st(1),st
bb: d9 c3                   fld    st(3)
bd: d8 4f dc                fmul   DWORD PTR [edi-0x24]
c0: de c1                   faddp  st(1),st
c2: d9 c4                   fld    st(4)
c4: d8 4f d8                fmul   DWORD PTR [edi-0x28]
c7: de c1                   faddp  st(1),st
c9: dd 1a                   fstp   QWORD PTR [edx]
cb: d9 c0                   fld    st(0)
cd: d8 4f d4                fmul   DWORD PTR [edi-0x2c]
d0: d9 c2                   fld    st(2)
d2: d8 4f d0                fmul   DWORD PTR [edi-0x30]
d5: de c1                   faddp  st(1),st
d7: d9 c3                   fld    st(3)
d9: d8 4f cc                fmul   DWORD PTR [edi-0x34]
dc: de c1                   faddp  st(1),st
de: d9 c4                   fld    st(4)
e0: d8 4f c8                fmul   DWORD PTR [edi-0x38]
e3: de c1                   faddp  st(1),st
e5: dd 19                   fstp   QWORD PTR [ecx]
e7: d8 4f c4                fmul   DWORD PTR [edi-0x3c]
ea: d9 c9                   fxch   st(1)
ec: d8 4f c0                fmul   DWORD PTR [edi-0x40]
ef: de c1                   faddp  st(1),st
f1: d9 c9                   fxch   st(1)
f3: d8 4f bc                fmul   DWORD PTR [edi-0x44]
f6: de c1                   faddp  st(1),st
f8: d9 c9                   fxch   st(1)
fa: d8 4f b8                fmul   DWORD PTR [edi-0x48]
fd: de c1                   faddp  st(1),st
ff: 83 c7 28                add    edi,0x28
102:    dd 87 70 ff ff ff       fld    QWORD PTR [edi-0x90]
108:    de e9                   fsubp  st(1),st
10a:    dd 47 f0                fld    QWORD PTR [edi-0x10]
10d:    dc c9                   fmul   st(1),st
10f:    d9 e8                   fld1
111:    de e9                   fsubp  st(1),st
113:    dc 4f d0                fmul   QWORD PTR [edi-0x30]
116:    de e9                   fsubp  st(1),st
118:    dd 59 08                fstp   QWORD PTR [ecx+0x8]
11b:    dd 01                   fld    QWORD PTR [ecx]
11d:    dc a7 78 ff ff ff       fsub   QWORD PTR [edi-0x88]
123:    dd 02                   fld    QWORD PTR [edx]
125:    dc 67 88                fsub   QWORD PTR [edi-0x78]
128:    dd 03                   fld    QWORD PTR [ebx]
12a:    dc 67 80                fsub   QWORD PTR [edi-0x80]
12d:    90                      nop
12e:    dd 47 f0                fld    QWORD PTR [edi-0x10]
131:    dd 86 e8 00 00 00       fld    QWORD PTR [esi+0xe8]
137:    d8 c9                   fmul   st,st(1)
139:    dd 9e e8 00 00 00       fstp   QWORD PTR [esi+0xe8]
13f:    90                      nop
140:    90                      nop
141:    dc ca                   fmul   st(2),st
143:    dc c9                   fmul   st(1),st
145:    d9 c0                   fld    st(0)
147:    d9 e8                   fld1
149:    de e9                   fsubp  st(1),st
14b:    d9 c0                   fld    st(0)
14d:    d9 c0                   fld    st(0)
14f:    dc 4f e8                fmul   QWORD PTR [edi-0x18]
152:    de ed                   fsubp  st(5),st
154:    dc 4f e0                fmul   QWORD PTR [edi-0x20]
157:    de eb                   fsubp  st(3),st
159:    d8 f1                   fdiv   st,st(1)
15b:    dc 4f d8                fmul   QWORD PTR [edi-0x28]
15e:    dc ec                   fsub   st(4),st
160:    d9 cc                   fxch   st(4)
162:    d9 e1                   fabs
164:    de ec                   fsubp  st(4),st
166:    de cb                   fmulp  st(3),st
168:    dd 1a                   fstp   QWORD PTR [edx]
16a:    dd 1b                   fstp   QWORD PTR [ebx]
16c:    dd 19                   fstp   QWORD PTR [ecx]
16e:    90                      nop
16f:    90                      nop
170:    90                      nop
171:    89 d8                   mov    eax,ebx
173:    5f                      pop    edi
174:    5e                      pop    esi
175:    5b                      pop    ebx
176:    5d                      pop    ebp
177:    c2 08 00                ret    0x8
```
