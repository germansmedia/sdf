# `ATetraVS2`

```
[OPTIONS]
.Version = 8
.DEscale = 0.2
.DEoption = 11
.RStop = 1024
.SIpower = 2
.Double Scale = 1.414
.Boxscale Min R = 0.5
.Double Fold = 1
.Double Scale vary = 0
.DRecipro Radius = 8
[CONSTANTS]
Double = -0.70710678118654752440084436210485
[CODE]
558BEC5657538B7D0889C38B773081C780000000837F50007F09FF4750DD46F0
DD5F48DD4748D9E8D9C1D9E1DEE1DC4ED0DEC1DD5F48DD46D8DC0EDD02D8C1DD
03D8C2D9C0D8C2D9EEDED9DFE0D0EC7714D9E0D8E2DD1AD9E0D8E1DD1BDDD8EB
3490909090D8E2D8E2D9C9D8E2D8E2D9C9D9C0D8C2D9EEDED9DFE0D0EC7210D9
E0D8C2DD1AD9E0D8C1DD1BDDD8EB06DDD8DDD8DDD8DD46D8DC0EDD01D8C1DD03
D8C2D9C0D8C2D9EEDED9DFE0D0EC7714D9E0D8E2DD19D9E0D8E1DD1BDDD8EB34
90909090D8E2D8E2D9C9D8E2D8E2D9C9D9C0D8C2D9EEDED9DFE0D0EC7210D9E0
D8C2DD19D9E0D8C1DD1BDDD8EB06DDD8DDD8DDD8DD46D8DC0EDD01D8C1DD02D8
C2D9C0D8C2D9EEDED9DFE0D0EC7714D9E0D8E2DD19D9E0D8E1DD1ADDD8EB3490
909090D8E2D8E2D9C9D8E2D8E2D9C9D9C0D8C2D9EEDED9DFE0D0EC7210D9E0D8
C2DD19D9E0D8C1DD1ADDD8EB06DDD8DDD8DDD8DD01DD02DD03D9C0D8C9D9C2D8
CBDEC1D9C3D8CCDEC1DC4EC8DC56E0DFE0D0EC7307DDD8DD46E8EB16D9E8D8D1
DFE0D0EC7207DDD8DC7F48EB05DED9DD4748DD4108D8C9DD5908DCCBDCCADEC9
DC47A8DD19DC47A0DD1ADC4798DD1B89D85B5F5E5DC20800
[END]

Revised and improved edition of Amazing Tetrahedron. More simple and visually correct results.
Based on this folding formula;

  Fold45 = - fold / sqrt(2)
  m = 0
  // folds the positive quadrant
  x = x + Fold45 // moves x to the first alignment
  y = y + Fold45 // moves y to the first alignment
  m = x+y
  if m>0
     n = x // does the fold
     x = -y
     y = -n
     x = x - Fold45
     y = y - Fold45
     else
         // folds the negative quadrant
         // we need a nested if to avoid a 4x fold, we need a triple repetition
         x = x - 2*Fold45 // moves x to the second alignment
         y = y - 2*Fold45 // moves y to the second alignment
         m = x+y
         if m<0
            n = x // does the 2nd fold
            x = -y
            y = -n
         endif
         x = x + Fold45 // and finally moves x and y back to the original spot
         y = y + Fold45
  endif
  (... do it for x+y, then x+z then y+z. Like in Sierpinski3 formula but it's a folding)
Luca GN 2011

-------

EDIT

may 23 2012 added radius. A great improvement in the fractal look is notable when radius is near to 10... Experiment by yourself!
```

## Disassembly

```
0:  55                      push   ebp
1:  8b ec                   mov    ebp,esp
3:  56                      push   esi
4:  57                      push   edi
5:  53                      push   ebx
6:  8b 7d 08                mov    edi,DWORD PTR [ebp+0x8]
9:  89 c3                   mov    ebx,eax
b:  8b 77 30                mov    esi,DWORD PTR [edi+0x30]
e:  81 c7 80 00 00 00       add    edi,0x80
14: 83 7f 50 00             cmp    DWORD PTR [edi+0x50],0x0
18: 7f 09                   jg     0x23
1a: ff 47 50                inc    DWORD PTR [edi+0x50]
1d: dd 46 f0                fld    QWORD PTR [esi-0x10]
20: dd 5f 48                fstp   QWORD PTR [edi+0x48]
23: dd 47 48                fld    QWORD PTR [edi+0x48]
26: d9 e8                   fld1
28: d9 c1                   fld    st(1)
2a: d9 e1                   fabs
2c: de e1                   fsubrp st(1),st
2e: dc 4e d0                fmul   QWORD PTR [esi-0x30]
31: de c1                   faddp  st(1),st
33: dd 5f 48                fstp   QWORD PTR [edi+0x48]
36: dd 46 d8                fld    QWORD PTR [esi-0x28]
39: dc 0e                   fmul   QWORD PTR [esi]
3b: dd 02                   fld    QWORD PTR [edx]
3d: d8 c1                   fadd   st,st(1)
3f: dd 03                   fld    QWORD PTR [ebx]
41: d8 c2                   fadd   st,st(2)
43: d9 c0                   fld    st(0)
45: d8 c2                   fadd   st,st(2)
47: d9 ee                   fldz
49: de d9                   fcompp
4b: df e0                   fnstsw ax
4d: d0 ec                   shr    ah,1
4f: 77 14                   ja     0x65
51: d9 e0                   fchs
53: d8 e2                   fsub   st,st(2)
55: dd 1a                   fstp   QWORD PTR [edx]
57: d9 e0                   fchs
59: d8 e1                   fsub   st,st(1)
5b: dd 1b                   fstp   QWORD PTR [ebx]
5d: dd d8                   fstp   st(0)
5f: eb 34                   jmp    0x95
61: 90                      nop
62: 90                      nop
63: 90                      nop
64: 90                      nop
65: d8 e2                   fsub   st,st(2)
67: d8 e2                   fsub   st,st(2)
69: d9 c9                   fxch   st(1)
6b: d8 e2                   fsub   st,st(2)
6d: d8 e2                   fsub   st,st(2)
6f: d9 c9                   fxch   st(1)
71: d9 c0                   fld    st(0)
73: d8 c2                   fadd   st,st(2)
75: d9 ee                   fldz
77: de d9                   fcompp
79: df e0                   fnstsw ax
7b: d0 ec                   shr    ah,1
7d: 72 10                   jb     0x8f
7f: d9 e0                   fchs
81: d8 c2                   fadd   st,st(2)
83: dd 1a                   fstp   QWORD PTR [edx]
85: d9 e0                   fchs
87: d8 c1                   fadd   st,st(1)
89: dd 1b                   fstp   QWORD PTR [ebx]
8b: dd d8                   fstp   st(0)
8d: eb 06                   jmp    0x95
8f: dd d8                   fstp   st(0)
91: dd d8                   fstp   st(0)
93: dd d8                   fstp   st(0)
95: dd 46 d8                fld    QWORD PTR [esi-0x28]
98: dc 0e                   fmul   QWORD PTR [esi]
9a: dd 01                   fld    QWORD PTR [ecx]
9c: d8 c1                   fadd   st,st(1)
9e: dd 03                   fld    QWORD PTR [ebx]
a0: d8 c2                   fadd   st,st(2)
a2: d9 c0                   fld    st(0)
a4: d8 c2                   fadd   st,st(2)
a6: d9 ee                   fldz
a8: de d9                   fcompp
aa: df e0                   fnstsw ax
ac: d0 ec                   shr    ah,1
ae: 77 14                   ja     0xc4
b0: d9 e0                   fchs
b2: d8 e2                   fsub   st,st(2)
b4: dd 19                   fstp   QWORD PTR [ecx]
b6: d9 e0                   fchs
b8: d8 e1                   fsub   st,st(1)
ba: dd 1b                   fstp   QWORD PTR [ebx]
bc: dd d8                   fstp   st(0)
be: eb 34                   jmp    0xf4
c0: 90                      nop
c1: 90                      nop
c2: 90                      nop
c3: 90                      nop
c4: d8 e2                   fsub   st,st(2)
c6: d8 e2                   fsub   st,st(2)
c8: d9 c9                   fxch   st(1)
ca: d8 e2                   fsub   st,st(2)
cc: d8 e2                   fsub   st,st(2)
ce: d9 c9                   fxch   st(1)
d0: d9 c0                   fld    st(0)
d2: d8 c2                   fadd   st,st(2)
d4: d9 ee                   fldz
d6: de d9                   fcompp
d8: df e0                   fnstsw ax
da: d0 ec                   shr    ah,1
dc: 72 10                   jb     0xee
de: d9 e0                   fchs
e0: d8 c2                   fadd   st,st(2)
e2: dd 19                   fstp   QWORD PTR [ecx]
e4: d9 e0                   fchs
e6: d8 c1                   fadd   st,st(1)
e8: dd 1b                   fstp   QWORD PTR [ebx]
ea: dd d8                   fstp   st(0)
ec: eb 06                   jmp    0xf4
ee: dd d8                   fstp   st(0)
f0: dd d8                   fstp   st(0)
f2: dd d8                   fstp   st(0)
f4: dd 46 d8                fld    QWORD PTR [esi-0x28]
f7: dc 0e                   fmul   QWORD PTR [esi]
f9: dd 01                   fld    QWORD PTR [ecx]
fb: d8 c1                   fadd   st,st(1)
fd: dd 02                   fld    QWORD PTR [edx]
ff: d8 c2                   fadd   st,st(2)
101:    d9 c0                   fld    st(0)
103:    d8 c2                   fadd   st,st(2)
105:    d9 ee                   fldz
107:    de d9                   fcompp
109:    df e0                   fnstsw ax
10b:    d0 ec                   shr    ah,1
10d:    77 14                   ja     0x123
10f:    d9 e0                   fchs
111:    d8 e2                   fsub   st,st(2)
113:    dd 19                   fstp   QWORD PTR [ecx]
115:    d9 e0                   fchs
117:    d8 e1                   fsub   st,st(1)
119:    dd 1a                   fstp   QWORD PTR [edx]
11b:    dd d8                   fstp   st(0)
11d:    eb 34                   jmp    0x153
11f:    90                      nop
120:    90                      nop
121:    90                      nop
122:    90                      nop
123:    d8 e2                   fsub   st,st(2)
125:    d8 e2                   fsub   st,st(2)
127:    d9 c9                   fxch   st(1)
129:    d8 e2                   fsub   st,st(2)
12b:    d8 e2                   fsub   st,st(2)
12d:    d9 c9                   fxch   st(1)
12f:    d9 c0                   fld    st(0)
131:    d8 c2                   fadd   st,st(2)
133:    d9 ee                   fldz
135:    de d9                   fcompp
137:    df e0                   fnstsw ax
139:    d0 ec                   shr    ah,1
13b:    72 10                   jb     0x14d
13d:    d9 e0                   fchs
13f:    d8 c2                   fadd   st,st(2)
141:    dd 19                   fstp   QWORD PTR [ecx]
143:    d9 e0                   fchs
145:    d8 c1                   fadd   st,st(1)
147:    dd 1a                   fstp   QWORD PTR [edx]
149:    dd d8                   fstp   st(0)
14b:    eb 06                   jmp    0x153
14d:    dd d8                   fstp   st(0)
14f:    dd d8                   fstp   st(0)
151:    dd d8                   fstp   st(0)
153:    dd 01                   fld    QWORD PTR [ecx]
155:    dd 02                   fld    QWORD PTR [edx]
157:    dd 03                   fld    QWORD PTR [ebx]
159:    d9 c0                   fld    st(0)
15b:    d8 c9                   fmul   st,st(1)
15d:    d9 c2                   fld    st(2)
15f:    d8 cb                   fmul   st,st(3)
161:    de c1                   faddp  st(1),st
163:    d9 c3                   fld    st(3)
165:    d8 cc                   fmul   st,st(4)
167:    de c1                   faddp  st(1),st
169:    dc 4e c8                fmul   QWORD PTR [esi-0x38]
16c:    dc 56 e0                fcom   QWORD PTR [esi-0x20]
16f:    df e0                   fnstsw ax
171:    d0 ec                   shr    ah,1
173:    73 07                   jae    0x17c
175:    dd d8                   fstp   st(0)
177:    dd 46 e8                fld    QWORD PTR [esi-0x18]
17a:    eb 16                   jmp    0x192
17c:    d9 e8                   fld1
17e:    d8 d1                   fcom   st(1)
180:    df e0                   fnstsw ax
182:    d0 ec                   shr    ah,1
184:    72 07                   jb     0x18d
186:    dd d8                   fstp   st(0)
188:    dc 7f 48                fdivr  QWORD PTR [edi+0x48]
18b:    eb 05                   jmp    0x192
18d:    de d9                   fcompp
18f:    dd 47 48                fld    QWORD PTR [edi+0x48]
192:    dd 41 08                fld    QWORD PTR [ecx+0x8]
195:    d8 c9                   fmul   st,st(1)
197:    dd 59 08                fstp   QWORD PTR [ecx+0x8]
19a:    dc cb                   fmul   st(3),st
19c:    dc ca                   fmul   st(2),st
19e:    de c9                   fmulp  st(1),st
1a0:    dc 47 a8                fadd   QWORD PTR [edi-0x58]
1a3:    dd 19                   fstp   QWORD PTR [ecx]
1a5:    dc 47 a0                fadd   QWORD PTR [edi-0x60]
1a8:    dd 1a                   fstp   QWORD PTR [edx]
1aa:    dc 47 98                fadd   QWORD PTR [edi-0x68]
1ad:    dd 1b                   fstp   QWORD PTR [ebx]
1af:    89 d8                   mov    eax,ebx
1b1:    5b                      pop    ebx
1b2:    5f                      pop    edi
1b3:    5e                      pop    esi
1b4:    5d                      pop    ebp
1b5:    c2 08 00                ret    0x8
```
