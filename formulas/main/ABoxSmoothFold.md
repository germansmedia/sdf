# `ABoxSmoothFold`

```
[OPTIONS]
.Version = 4
.DEscale = .2
.DEoption = 11
.RStop = 1024
.SIpower = 2
.Double Scale =   2
.Boxscale Min R = 0.5
.Double Fold = 1
.Double Scale vary = 0
.Integer Sharpness (Integer 2+) = 6
.Double Fix (BoxFold) = 1
.Integer Sh. of BallFold (Int 3+) = 4
.Double Fix (BallFold) = .3
[CONSTANTS]
Double = .99
[CODE]
558BEC5657538B75088B7E3089C383C670EB4E909090D1E874167304D9C0EB04
D9E8D9C9D8C8D1E873FADCC975F6DDD8C3DD00D9E1508B47CCD9C0E8D6FFFFFF
58DC4FC4D9C9DD47D8D8C0D8E1D8CADEC1D9E8DEC2DEF1F64007807902D9E0C3
90E8CBFFFFFF89D0E8C4FFFFFF89C8E8BDFFFFFF837E60007F09FF4660DD47F0
DD5E58DD4658D9E8D9C1D9E1DEE1DC4FD0DEC1DD5E58DD47E0DC1F9BDFE0D0EC
7204D9E8EB5AD9C2D8C8D9C2D8C8DEC1D9C1D8C8DEC1D9E8DC47E0DC4FF8D9E8
DC67E0DC4FF8D9CAD8E1D9E49BDFE050D9E1D8F2D9C08B47C0D1E87304D9FAD1
D0E830FFFFFFDC4FB8D9E8D8C1D9CADEC1DEF158DECAD9C9D0EC7302D9E0DEC1
DD4658DEF1DD4108D8C9DD5908DCCBDCCADEC9DC46B8DD19DC46B0DD1ADC46A8
DD1B89D85B5F5E5DC20800
[END]

WARNING! If you use very high sharpness, I don't know what will happen (it may become
extremely slow)... But it has no sense
An Amazing Box, with Folding function modified as follows (x is each spatial coord);

 xp = fix1*x**p; (fast int pow, p is forced to be > 2)
 u=sgn(x)*xp+1;
 x=x+xp*(2*Fold-x);
 x=x/u;

Also radius folding is smooth (when minr<0.99);

 if minr>0.99
 r=1; // prevents fp troubles and speeds up a lot in this case
 else
 r = (x*x+y*y+z*z);
 m=(1+minr)/2; n=(1-minr)/2; r1=(r-m)/n; rs=sgn(r1); r1=abs(r1);
 rsqrt = sqrt(r1); // this slows down the convergence
 rp=fix2*(rsqrt**p); (fast int pow, p is forced to be > 3)
 r = rs*((rp+r1)/(rp+1) * n) + m;
 endif

 (then r = scale / r and is used as a multiplier for x,y,z)

This "smoothens" the folding function, replacing the "sawtooth" fold with a smooth curve
Result is very similar to standard ABox but with a bit of detail loss and "curvy" details...
Higher sharpness -> You get closer and closer to standard ABox...
Also, remember that smoothing has an interference with the scale. More smooth -> less scale
Inspired from an idea of Buddhi @ FractalForums.
Interesting results can be achieved with negative fix values (more smoothness, different look).

Luca GN 2011

19.03.2013 Jesse speedup modification
```

## Disassembly

```
0:  55                      push   ebp
1:  8b ec                   mov    ebp,esp
3:  56                      push   esi
4:  57                      push   edi
5:  53                      push   ebx
6:  8b 75 08                mov    esi,DWORD PTR [ebp+0x8]
9:  8b 7e 30                mov    edi,DWORD PTR [esi+0x30]
c:  89 c3                   mov    ebx,eax
e:  83 c6 70                add    esi,0x70
11: eb 4e                   jmp    0x61
13: 90                      nop
14: 90                      nop
15: 90                      nop
16: d1 e8                   shr    eax,1
18: 74 16                   je     0x30
1a: 73 04                   jae    0x20
1c: d9 c0                   fld    st(0)
1e: eb 04                   jmp    0x24
20: d9 e8                   fld1
22: d9 c9                   fxch   st(1)
24: d8 c8                   fmul   st,st(0)
26: d1 e8                   shr    eax,1
28: 73 fa                   jae    0x24
2a: dc c9                   fmul   st(1),st
2c: 75 f6                   jne    0x24
2e: dd d8                   fstp   st(0)
30: c3                      ret
31: dd 00                   fld    QWORD PTR [eax]
33: d9 e1                   fabs
35: 50                      push   eax
36: 8b 47 cc                mov    eax,DWORD PTR [edi-0x34]
39: d9 c0                   fld    st(0)
3b: e8 d6 ff ff ff          call   0x16
40: 58                      pop    eax
41: dc 4f c4                fmul   QWORD PTR [edi-0x3c]
44: d9 c9                   fxch   st(1)
46: dd 47 d8                fld    QWORD PTR [edi-0x28]
49: d8 c0                   fadd   st,st(0)
4b: d8 e1                   fsub   st,st(1)
4d: d8 ca                   fmul   st,st(2)
4f: de c1                   faddp  st(1),st
51: d9 e8                   fld1
53: de c2                   faddp  st(2),st
55: de f1                   fdivrp st(1),st
57: f6 40 07 80             test   BYTE PTR [eax+0x7],0x80
5b: 79 02                   jns    0x5f
5d: d9 e0                   fchs
5f: c3                      ret
60: 90                      nop
61: e8 cb ff ff ff          call   0x31
66: 89 d0                   mov    eax,edx
68: e8 c4 ff ff ff          call   0x31
6d: 89 c8                   mov    eax,ecx
6f: e8 bd ff ff ff          call   0x31
74: 83 7e 60 00             cmp    DWORD PTR [esi+0x60],0x0
78: 7f 09                   jg     0x83
7a: ff 46 60                inc    DWORD PTR [esi+0x60]
7d: dd 47 f0                fld    QWORD PTR [edi-0x10]
80: dd 5e 58                fstp   QWORD PTR [esi+0x58]
83: dd 46 58                fld    QWORD PTR [esi+0x58]
86: d9 e8                   fld1
88: d9 c1                   fld    st(1)
8a: d9 e1                   fabs
8c: de e1                   fsubrp st(1),st
8e: dc 4f d0                fmul   QWORD PTR [edi-0x30]
91: de c1                   faddp  st(1),st
93: dd 5e 58                fstp   QWORD PTR [esi+0x58]
96: dd 47 e0                fld    QWORD PTR [edi-0x20]
99: dc 1f                   fcomp  QWORD PTR [edi]
9b: 9b df e0                fstsw  ax
9e: d0 ec                   shr    ah,1
a0: 72 04                   jb     0xa6
a2: d9 e8                   fld1
a4: eb 5a                   jmp    0x100
a6: d9 c2                   fld    st(2)
a8: d8 c8                   fmul   st,st(0)
aa: d9 c2                   fld    st(2)
ac: d8 c8                   fmul   st,st(0)
ae: de c1                   faddp  st(1),st
b0: d9 c1                   fld    st(1)
b2: d8 c8                   fmul   st,st(0)
b4: de c1                   faddp  st(1),st
b6: d9 e8                   fld1
b8: dc 47 e0                fadd   QWORD PTR [edi-0x20]
bb: dc 4f f8                fmul   QWORD PTR [edi-0x8]
be: d9 e8                   fld1
c0: dc 67 e0                fsub   QWORD PTR [edi-0x20]
c3: dc 4f f8                fmul   QWORD PTR [edi-0x8]
c6: d9 ca                   fxch   st(2)
c8: d8 e1                   fsub   st,st(1)
ca: d9 e4                   ftst
cc: 9b df e0                fstsw  ax
cf: 50                      push   eax
d0: d9 e1                   fabs
d2: d8 f2                   fdiv   st,st(2)
d4: d9 c0                   fld    st(0)
d6: 8b 47 c0                mov    eax,DWORD PTR [edi-0x40]
d9: d1 e8                   shr    eax,1
db: 73 04                   jae    0xe1
dd: d9 fa                   fsqrt
df: d1 d0                   rcl    eax,1
e1: e8 30 ff ff ff          call   0x16
e6: dc 4f b8                fmul   QWORD PTR [edi-0x48]
e9: d9 e8                   fld1
eb: d8 c1                   fadd   st,st(1)
ed: d9 ca                   fxch   st(2)
ef: de c1                   faddp  st(1),st
f1: de f1                   fdivrp st(1),st
f3: 58                      pop    eax
f4: de ca                   fmulp  st(2),st
f6: d9 c9                   fxch   st(1)
f8: d0 ec                   shr    ah,1
fa: 73 02                   jae    0xfe
fc: d9 e0                   fchs
fe: de c1                   faddp  st(1),st
100:    dd 46 58                fld    QWORD PTR [esi+0x58]
103:    de f1                   fdivrp st(1),st
105:    dd 41 08                fld    QWORD PTR [ecx+0x8]
108:    d8 c9                   fmul   st,st(1)
10a:    dd 59 08                fstp   QWORD PTR [ecx+0x8]
10d:    dc cb                   fmul   st(3),st
10f:    dc ca                   fmul   st(2),st
111:    de c9                   fmulp  st(1),st
113:    dc 46 b8                fadd   QWORD PTR [esi-0x48]
116:    dd 19                   fstp   QWORD PTR [ecx]
118:    dc 46 b0                fadd   QWORD PTR [esi-0x50]
11b:    dd 1a                   fstp   QWORD PTR [edx]
11d:    dc 46 a8                fadd   QWORD PTR [esi-0x58]
120:    dd 1b                   fstp   QWORD PTR [ebx]
122:    89 d8                   mov    eax,ebx
124:    5b                      pop    ebx
125:    5f                      pop    edi
126:    5e                      pop    esi
127:    5d                      pop    ebp
128:    c2 08 00                ret    0x8
```