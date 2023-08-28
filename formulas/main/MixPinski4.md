# `MixPinski4`

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
[CODE]
558BEC53565789C38B75088B7E30DD41
08DD01DD02DD0390D9E0D8D1DFE0D0EC
7202D9C9D8D2DFE0D0EC7202D9CAD9E0
D9C9D9E0D8D2DFE0D0EC7202D9CAD9E0
D9C990D9E0D8D3DFE0D0EC7202D9CBD9
E0D9C9D9E0D8D3DFE0D0EC7202D9CBD9
E0D9C9D9CAD9E0D8D3DFE0D0EC7202D9
CBD9E0D9CA9083EF28D9C0D84FF4D9C2
D84FF0DEC1D9C3D84FECDEC1D9C4D84F
E8DEC1DD1BD9C0D84FE4D9C2D84FE0DE
C1D9C3D84FDCDEC1D9C4D84FD8DEC1DD
1AD9C0D84FD4D9C2D84FD0DEC1D9C3D8
4FCCDEC1D9C4D84FC8DEC1DD19D84FC4
D9C9D84FC0DEC1D9C9D84FBCDEC1D9C9
D84FB8DEC183C728DD47F0DCC9D9E8DE
E9DC4FD0DEE9DD5908DD01DD02DD03DD
47F0DD86E8000000D8C9DD9EE8000000
9090DCCADCC9D9C0D9E8DEE9D9C0D9C0
DC4FE8DEEDDC4FE0DEEBD8F1DC4FD8DC
ECD9CCD9E1DEECDECBDD1ADD1BDD1990
909089D85F5E5B5DC20800
[END]

Description:

NOTE: If the formula does not render correctly together with 3D formulas check "Disable analytical DE".

A strange but intriguing fractal, that mixes Sierpinski and Menger folds. The amazing thing is that in 3D it does not work so well!

MixPinski4(x,y,z,w){
   r=x*x+y*y+z*z;
   for(i=0;i<MI && r<bailout;i++){

      if(z.x+z.y<0.0) z.xy = -z.yx;
      if(z.x+z.z<0.0) z.xz = -z.zx;
      if(z.y+z.z<0.0) z.zy = -z.yz;
      if(z.x+z.w<0.0) z.xw = -z.wx;
      if(z.y+z.w<0.0) z.yw = -z.wy;
      if(z.z+z.w<0.0) z.zw = -z.wz;

      rotate4D(x,y,z,w);

      x=scale*x-CX*(scale-1);
      y=scale*y-CY*(scale-1);
      w=scale*w-CW*(scale-1);
      z-=0.5*CZ*(scale-1)/scale;
      z=-abs(-z);
      z+=0.5*CZ*(scale-1)/scale;
      z=scale*z;
      
      r=x*x+y*y+z*z;
   }
   return sqrt(x*x+y*y+z*z)*scale^(-i);

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
75: 90                      nop
76: 83 ef 28                sub    edi,0x28
79: d9 c0                   fld    st(0)
7b: d8 4f f4                fmul   DWORD PTR [edi-0xc]
7e: d9 c2                   fld    st(2)
80: d8 4f f0                fmul   DWORD PTR [edi-0x10]
83: de c1                   faddp  st(1),st
85: d9 c3                   fld    st(3)
87: d8 4f ec                fmul   DWORD PTR [edi-0x14]
8a: de c1                   faddp  st(1),st
8c: d9 c4                   fld    st(4)
8e: d8 4f e8                fmul   DWORD PTR [edi-0x18]
91: de c1                   faddp  st(1),st
93: dd 1b                   fstp   QWORD PTR [ebx]
95: d9 c0                   fld    st(0)
97: d8 4f e4                fmul   DWORD PTR [edi-0x1c]
9a: d9 c2                   fld    st(2)
9c: d8 4f e0                fmul   DWORD PTR [edi-0x20]
9f: de c1                   faddp  st(1),st
a1: d9 c3                   fld    st(3)
a3: d8 4f dc                fmul   DWORD PTR [edi-0x24]
a6: de c1                   faddp  st(1),st
a8: d9 c4                   fld    st(4)
aa: d8 4f d8                fmul   DWORD PTR [edi-0x28]
ad: de c1                   faddp  st(1),st
af: dd 1a                   fstp   QWORD PTR [edx]
b1: d9 c0                   fld    st(0)
b3: d8 4f d4                fmul   DWORD PTR [edi-0x2c]
b6: d9 c2                   fld    st(2)
b8: d8 4f d0                fmul   DWORD PTR [edi-0x30]
bb: de c1                   faddp  st(1),st
bd: d9 c3                   fld    st(3)
bf: d8 4f cc                fmul   DWORD PTR [edi-0x34]
c2: de c1                   faddp  st(1),st
c4: d9 c4                   fld    st(4)
c6: d8 4f c8                fmul   DWORD PTR [edi-0x38]
c9: de c1                   faddp  st(1),st
cb: dd 19                   fstp   QWORD PTR [ecx]
cd: d8 4f c4                fmul   DWORD PTR [edi-0x3c]
d0: d9 c9                   fxch   st(1)
d2: d8 4f c0                fmul   DWORD PTR [edi-0x40]
d5: de c1                   faddp  st(1),st
d7: d9 c9                   fxch   st(1)
d9: d8 4f bc                fmul   DWORD PTR [edi-0x44]
dc: de c1                   faddp  st(1),st
de: d9 c9                   fxch   st(1)
e0: d8 4f b8                fmul   DWORD PTR [edi-0x48]
e3: de c1                   faddp  st(1),st
e5: 83 c7 28                add    edi,0x28
e8: dd 47 f0                fld    QWORD PTR [edi-0x10]
eb: dc c9                   fmul   st(1),st
ed: d9 e8                   fld1
ef: de e9                   fsubp  st(1),st
f1: dc 4f d0                fmul   QWORD PTR [edi-0x30]
f4: de e9                   fsubp  st(1),st
f6: dd 59 08                fstp   QWORD PTR [ecx+0x8]
f9: dd 01                   fld    QWORD PTR [ecx]
fb: dd 02                   fld    QWORD PTR [edx]
fd: dd 03                   fld    QWORD PTR [ebx]
ff: dd 47 f0                fld    QWORD PTR [edi-0x10]
102:    dd 86 e8 00 00 00       fld    QWORD PTR [esi+0xe8]
108:    d8 c9                   fmul   st,st(1)
10a:    dd 9e e8 00 00 00       fstp   QWORD PTR [esi+0xe8]
110:    90                      nop
111:    90                      nop
112:    dc ca                   fmul   st(2),st
114:    dc c9                   fmul   st(1),st
116:    d9 c0                   fld    st(0)
118:    d9 e8                   fld1
11a:    de e9                   fsubp  st(1),st
11c:    d9 c0                   fld    st(0)
11e:    d9 c0                   fld    st(0)
120:    dc 4f e8                fmul   QWORD PTR [edi-0x18]
123:    de ed                   fsubp  st(5),st
125:    dc 4f e0                fmul   QWORD PTR [edi-0x20]
128:    de eb                   fsubp  st(3),st
12a:    d8 f1                   fdiv   st,st(1)
12c:    dc 4f d8                fmul   QWORD PTR [edi-0x28]
12f:    dc ec                   fsub   st(4),st
131:    d9 cc                   fxch   st(4)
133:    d9 e1                   fabs
135:    de ec                   fsubp  st(4),st
137:    de cb                   fmulp  st(3),st
139:    dd 1a                   fstp   QWORD PTR [edx]
13b:    dd 1b                   fstp   QWORD PTR [ebx]
13d:    dd 19                   fstp   QWORD PTR [ecx]
13f:    90                      nop
140:    90                      nop
141:    90                      nop
142:    89 d8                   mov    eax,ebx
144:    5f                      pop    edi
145:    5e                      pop    esi
146:    5b                      pop    ebx
147:    5d                      pop    ebp
148:    c2 08 00                ret    0x8
```
