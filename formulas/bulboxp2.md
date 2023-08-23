# BulboxP-2

```
[OPTIONS]
.Version = 2
.DEscale = 1
.SIpower = 2
.RStop = 1024
.SIpower = 2
.Double Scale = 2
.Double Fold = 1
.Double Inner R = .6
.Double Inner Scale = -.5
.Double Unsharpening = 0
.Double Inv-xCen = 0
.Double Inv-yCen = 0
.Double Inv-zCen = 0
.Double Inner Z Mul = 1
.Integer Disable box = 0
[CODE]
558BEC81EC800000005356578B75088B7E308BD8D9D0DD03DC47E8D9E1DD03DC
67E8D9E1D9E0DC23DEC1DD1BDD02DC47E8D9E1DD02DC67E8D9E1D9E0DC22DEC1
DD1ADD01DC47E8D9E1DD01DC67E8D9E1D9E0DC21DEC1DD19DD03D8C8DD02D8C8
DD01D8C8DEC1DEC1DD55E8D9FADD5DE0DD47F0DD03D8C9DC67C8DD1BDD02D8C9
DC67C0DD1ADD01D8C9DC67B8DD19DDD8837FAC00751590DD45E0D9E8D9C9DED9
DFE0D0EC0F83C900000090DD45E0DD47E0DED9DFE0D0EC0F83D10000009090DD
45E0DC67E0D9E8DC67E0DEF9DD55D8D9E8D8E1DD5DD0DDD890DD02D8C8DD03D8
C8DEC1DD47D0D9E1DEC1D9FADD5DF89090DD45E8D8C8DD47D8DEF1DD5DE890DD
01DC75F8D8C8D9E0D9E8DEC1DD5DF090DD03D8C8DD02D8C8D9E0DEC1DC4DF0DC
4DE890DD03DC0AD9E0DC4DF0DC4DE890DD01D8C0D9E0DC4FB0DC4DF8DC4DE890
DC4DD0DD01DC4DD8DEC1DC4628DD19DC4DD0DD02DC4DD8DEC1DC4620DD1ADC4D
D0DD03DC4DD8DEC1DC4618DD1BE99500000090DD03DC4618DD1BDD02DC4620DD
1ADD01DC4628DD19E97A00000090DD02D8C8DD03D8C8DEC1DD47D0D9E1DEC1D9
FADD5DF890DD45E8D8C8DD47D8DEF1DD5DE890DD01DC75F8D8C8D9E0D9E8DEC1
DD5DF090DD03D8C8DD02D8C8D9E0DEC1DC4DF0DC4DE8DC461890DD03DC0AD9E0
DC4DF0DC4DE8DC462090DD01D8C0D9E0DC4DF8DC4DE8DC4FB0DC462890DD19DD
1ADD1B90909090DD03DC47C8DD1BDD02DC47C0DD1ADD01DC47B8DD19D9D08BC3
5F5E5B89EC5DC20800
[END]

This formula is an experiment with the W/N negative powers.
And, due to its "weirdness", it requires a very low stepwidth/raystep to show up correctly.
Don't use a SmallR next to 1, or (even more) weird things can happen.
This is the concept;

- At first do a Tglad folding
- Calculate radius = cabs(x,y,z)
- Scale the vector
- Raise to power -2 the vector (according to W/N theory), and store in Tvec
// W/N power -2 formula is (with unsharpening factor);
// Tvec = [ ( x*x - y*y ) a ; -2*x*y*a ; -2*z*rxy*izM ] * pow(radius,-4)
// and rxy = sqrt(x*x+y*y+Unsh), a = 1 - (z/rxy)^2
// (Yes both z and y have negative sign, that's why a neg ISc is better)
- If radius > 1, just add C. (Gives the boxy look, never executed in "Disable box" mode)
- ElseIf radius < InR, vector = ISc*Tvec + Cvec (apply the exponentiation)
- Else; (interpolation, necessary to remove branch cuts)
  k = (R-InR)/(1-InR)
  x = ISc*Tvecx * (1-k) + oldx * k + Cx
  y = ISc*Tvecy * (1-k) + oldy * k + Cy
  z = ISc*Tvecz * (1-k) + oldz * k + Cz

- Now multiply the vector by the scale
- Finally add C

The result is full of discontinuities and odd, but you might like it.

Luca GN 2011
```

### Disassembly

[ebx] = x
[edx] = y
[ecx] = z
[ecx+0x8] = w

[edi-0x10] = scale (s)
[edi-0x18] = fold (f)
[edi-0x20] = inner r (r)
[edi-0x28] = inner scale (is)
[edi-0x30] = unsharpening (u)
[edi-0x38] = inv-xcen (ix)
[edi-0x40] = inv-ycen (iy)
[edi-0x48] = inv-zcen (iz)
[edi-0x50] = inner z mul (izm)
[edi-0x54] = disable box (d)

[esi+0x18] = cx
[esi+0x20] = cy
[esi+0x28] = cz

[ebp-0x8] = t0
[ebp-0x10] = t1
[ebp-0x18] = t2
[ebp-0x20] = t3
[ebp-0x28] = t4
[ebp-0x30] = t5

```
0:  55                      push   ebp
1:  8b ec                   mov    ebp,esp

3:  81 ec 80 00 00 00       sub    esp,0x80

9:  53                      push   ebx

a:  56                      push   esi
b:  57                      push   edi
c:  8b 75 08                mov    esi,DWORD PTR [ebp+0x8]
f:  8b 7e 30                mov    edi,DWORD PTR [esi+0x30]

12: 8b d8                   mov    ebx,eax

14: d9 d0                   fnop

16: dd 03                   fld    x               // x;
18: dc 47 e8                fadd   f               // x+f;
1b: d9 e1                   fabs                   // |x+f|;
1d: dd 03                   fld    x               // x; |x+f|;
1f: dc 67 e8                fsub   f               // x-f; |x+f|;
22: d9 e1                   fabs                   // |x-f|; |x+f|;
24: d9 e0                   fchs                   // -|x-f|; |x+f|;
26: dc 23                   fsub   x               // -|x-f|-x; |x+f|;
28: de c1                   faddp  st(1),st        // |x+f|-|x-f|+x;
2a: dd 1b                   fstp   x               // x = |x + f| - |x - f| + x

2c: dd 02                   fld    y               // y;
2e: dc 47 e8                fadd   f               // y+f;
31: d9 e1                   fabs                   // |y+f|;
33: dd 02                   fld    y               // y; |y+f|;
35: dc 67 e8                fsub   f               // y-f; |y+f|;
38: d9 e1                   fabs                   // |y-f|; |y+f|;
3a: d9 e0                   fchs                   // -|y-f|; |y+f|;
3c: dc 22                   fsub   y               // -|y-f|-y; |y+f|;
3e: de c1                   faddp  st(1),st        // |y+f|-|y-f|+y;
40: dd 1a                   fstp   y               // y = |y + f| - |y - f| + y

42: dd 01                   fld    z               // z;
44: dc 47 e8                fadd   f               // z+f;
47: d9 e1                   fabs                   // |z+f|;
49: dd 01                   fld    z               // z; |z+f|;
4b: dc 67 e8                fsub   f               // z-f; |z+f|;
4e: d9 e1                   fabs                   // |z-f|; |z+f|;
50: d9 e0                   fchs                   // -|z-f|; |z+f|;
52: dc 21                   fsub   z               // -|z-f|-z; |z+f|;
54: de c1                   faddp  st(1),st        // |z+f|-|z-f|+z;
56: dd 19                   fstp   z               // z = |z + f| - |z - f| + z;

58: dd 03                   fld    x               // x;
5a: d8 c8                   fmul   st,st(0)        // x*x;
5c: dd 02                   fld    y               // y; x*x;
5e: d8 c8                   fmul   st,st(0)        // y*y; x*x;
60: dd 01                   fld    z               // z; y*y; x*x;
62: d8 c8                   fmul   st,st(0)        // z*z; y*y; x*x;
64: de c1                   faddp  st(1),st        // y*y+z*z; x*x;
66: de c1                   faddp  st(1),st        // x*x+y*y+z*z;
68: dd 55 e8                fst    t2              // t2 = x * x + y * y + z * z, x*x+y*y+z*z;
6b: d9 fa                   fsqrt                  // sqrt(x*x+y*y+z*z);
6d: dd 5d e0                fstp   t3              // t3 = sqrt(t2)

70: dd 47 f0                fld    s               // s;

73: dd 03                   fld    x               // x; s;
75: d8 c9                   fmul   st,st(1)        // x*s; s;
77: dc 67 c8                fsub   ix              // x*s-ix; s;
7a: dd 1b                   fstp   x               // x = x * s - ix, s;

7c: dd 02                   fld    y               // y; s;
7e: d8 c9                   fmul   st,st(1)        // y*s; s;
80: dc 67 c0                fsub   iy              // y*s-iy; s;
83: dd 1a                   fstp   y               // y = y * s - iy, s;

85: dd 01                   fld    z               // z; s;
87: d8 c9                   fmul   st,st(1)        // z*s; s; 
89: dc 67 b8                fsub   iz              // z*s-iz; s;
8c: dd 19                   fstp   z               // z = z * s - iz; s;

8e: dd d8                   fstp   st(0)

90: 83 7f ac 00             cmp    d,0x0           // d ? 0
94: 75 15                   jne    0xab

96: 90                      nop

d == 0:
97: dd 45 e0                fld    t3              // t3;
9a: d9 e8                   fld1                   // 1; t3;
9c: d9 c9                   fxch   st(1)           // t3; 1;
9e: de d9                   fcompp                 // t3 ? 1
a0: df e0                   fnstsw ax
a2: d0 ec                   shr    ah,1
a4: 0f 83 c9 00 00 00       jae    0x173

(d == 0) && (t3 < 1):
aa: 90                      nop

((d == 0) && (t3 < 1)) || (d != 0):
ab: dd 45 e0                fld    t3              // t3;
ae: dd 47 e0                fld    r               // r; t3;
b1: de d9                   fcompp                 // r ? t3
b3: df e0                   fnstsw ax
b5: d0 ec                   shr    ah,1
b7: 0f 83 d1 00 00 00       jae    0x18e

(((d == 0) && (t3 < 1)) || (d != 0)) && (t3 > r):
bd: 90                      nop
be: 90                      nop

bf: dd 45 e0                fld    t3              // t3;
c2: dc 67 e0                fsub   r               // t3-r;
c5: d9 e8                   fld1                   // 1; t3-r;
c7: dc 67 e0                fsub   r               // 1-r; t3-r;
ca: de f9                   fdivp  st(1),st        // (t3-r)/(1-r);
cc: dd 55 d8                fst    t4              // t4 = (t3 - r) / (1 - r), (t3-r)/(1-r);

cf: d9 e8                   fld1                   // 1; (t3-r)/(1-r);
d1: d8 e1                   fsub   st,st(1)        // 1-(t3-r)/(1-r); (t3-r)/(1-r);
d3: dd 5d d0                fstp   t5              // t5 = 1 - (t3 - r) / (1 - r), (t3-r)/(1-r);
d6: dd d8                   fstp   st(0)

d8: 90                      nop

d9: dd 02                   fld    y               // y;
db: d8 c8                   fmul   st,st(0)        // y*y;
dd: dd 03                   fld    x               // x; y*y;
df: d8 c8                   fmul   st,st(0)        // x*x; y*y;
e1: de c1                   faddp  st(1),st        // y*y+x*x;
e3: dd 47 d0                fld    u               // u; y*y+x*x;
e6: d9 e1                   fabs                   // |u|; y*y+x*x;
e8: de c1                   faddp  st(1),st        // y*y+x*x+|u|;
ea: d9 fa                   fsqrt                  // sqrt(y*y+x*x+|u|);
ec: dd 5d f8                fstp   t0              // t0 = sqrt(y * y + x * x + |u|)

ef: 90                      nop
f0: 90                      nop

f1: dd 45 e8                fld    t2              // t2;
f4: d8 c8                   fmul   st,st(0)        // t2*t2;
f6: dd 47 d8                fld    is              // is; t2*t2;
f9: de f1                   fdivrp st(1),st        // is/(t2*t2);
fb: dd 5d e8                fstp   t2              // t2 = is / (t2 * t2)

fe: 90                      nop

ff: dd 01                   fld    z               // z;
101:    dc 75 f8            fdiv   t0              // z/t0;
104:    d8 c8               fmul   st,st(0)        // (z*z)/(t0*t0);
106:    d9 e0               fchs                   // -(z*z)/(t0*t0);
108:    d9 e8               fld1                   // 1; -(z*z)/(t0*t0);
10a:    de c1               faddp  st(1),st        // -(z*z)/(t0*t0)+1;
10c:    dd 5d f0            fstp   t1              // t1 = -(z * z) / (t0 * t0) + 1

10f:    90                  nop

110:    dd 03               fld    x               // x;
112:    d8 c8               fmul   st,st(0)        // x*x;
114:    dd 02               fld    y               // y; x*x;
116:    d8 c8               fmul   st,st(0)        // y*y; x*x;
118:    d9 e0               fchs                   // -y*y; x*x;
11a:    de c1               faddp  st(1),st        // x*x-y*y;
11c:    dc 4d f0            fmul   t1              // (x*x-y*y)*t1;
11f:    dc 4d e8            fmul   t2              // (x*x-y*y)*t1*t2;

122:    90                  nop

123:    dd 03               fld    x               // x; (x*x-y*y)*t1*t2;
125:    dc 0a               fmul   y               // x*y; (x*x-y*y)*t1*t2;
127:    d9 e0               fchs                   // -x*y; (x*x-y*y)*t1*t2;
129:    dc 4d f0            fmul   t1              // -x*y*t1; (x*x-y*y)*t1*t2;
12c:    dc 4d e8            fmul   t2              // -x*y*t2; (x*x-y*y)*t1*t2;

12f:    90                  nop

130:    dd 01               fld    z               // z; -x*y*t2; (x*x-y*y)*t1*t2;
132:    d8 c0               fadd   st,st(0)        // 2*z; -x*y*t2; (x*x-y*y)*t1*t2;
134:    d9 e0               fchs                   // -2*z; -x*y*t2; (x*x-y*y)*t1*t2;
136:    dc 4f b0            fmul   izm             // -2*z*izm; -x*y*t2; (x*x-y*y)*t1*t2;
139:    dc 4d f8            fmul   t0              // -2*z*izm*t0; -x*y*t2; (x*x-y*y)*t1*t2;
13c:    dc 4d e8            fmul   t2              // -2*z*izm*t0*t2; -x*y*t2; (x*x-y*y)*t1*t2;

13f:    90                  nop

140:    dc 4d d0            fmul   t5              // -2*z*izm*t0*t2*t5; -x*y*t2; (x*x-y*y)*t1*t2;

143:    dd 01               fld    z               // z; -2*z*izm*t0*t2*t5; -x*y*t2; (x*x-y*y)*t1*t2;
145:    dc 4d d8            fmul   t4              // z*t4; -2*z*izm*t0*t2*t5; -x*y*t2; (x*x-y*y)*t1*t2;
148:    de c1               faddp  st(1),st        // -2*z*izm*t0*t2*t5+z*t4; -x*y*t2; (x*x-y*y)*t1*t2;
14a:    dc 46 28            fadd   cz              // -2*z*izm*t0*t2*t5+z*t4+cz; -x*y*t2; (x*x-y*y)*t1*t2;
14d:    dd 19               fstp   z               // z = -2 * z * izm * t0 * t2 * t5 + z * t4 + cz, -x*y*t2; (x*x-y*y)*t1*t2;

14f:    dc 4d d0            fmul   t5              // -x*y*t2*t5; (x*x-y*y)*t1*t2;
152:    dd 02               fld    y               // y; -x*y*t2*t5; (x*x-y*y)*t1*t2;
154:    dc 4d d8            fmul   t4              // y*t4; -x*y*t2*t5; (x*x-y*y)*t1*t2;
157:    de c1               faddp  st(1),st        // -x*y*t2*t5+y*t4; (x*x-y*y)*t1*t2;
159:    dc 46 20            fadd   cy              // -x*y*t2*t5+y*t4+cy; (x*x-y*y)*t1*t2;
15c:    dd 1a               fstp   y               // y = -x * y * t2 * t5 + y * t4 + cy, (x*x-y*y)*t1*t2;

15e:    dc 4d d0            fmul   t5              // (x*x-y*y)*t1*t2*t5;
161:    dd 03               fld    x               // x; (x*x-y*y)*t1*t2*t5;
163:    dc 4d d8            fmul   t4              // x*t4; (x*x-y*y)*t1*t2*t5;
166:    de c1               faddp  st(1),st        // (x*x-y*y)*t1*t2*t5+x*t4;
168:    dc 46 18            fadd   cx              // (x*x-y*y)*t1*t2*t5+x*t4+cx;
16b:    dd 1b               fstp   x               // x = (x * x - y * y) * t1 * t2 * t5 + x * t4 + cx
16d:    e9 95 00 00 00      jmp    0x207

172:    90                  nop

(d == 0) && (t3 >= 1):
173:    dd 03               fld    x               // x;
175:    dc 46 18            fadd   cx              // x+cx;
178:    dd 1b               fstp   x               // x = x + cx

17a:    dd 02               fld    y               // y;
17c:    dc 46 20            fadd   cy              // y+cy;
17f:    dd 1a               fstp   y               // y = y + cy

181:    dd 01               fld    z               // z;
183:    dc 46 28            fadd   cz              // z+cz;
186:    dd 19               fstp   z               // z = z + cz

188:    e9 7a 00 00 00      jmp    0x207

18d:    90                  nop

(((d == 0) && (t3 < 1)) || (d != 0)) && (t3 <= r):
18e:    dd 02               fld    y               // y;
190:    d8 c8               fmul   st,st(0)        // y*y;
192:    dd 03               fld    x               // x; y*y;
194:    d8 c8               fmul   st,st(0)        // x*x; y*y;
196:    de c1               faddp  st(1),st        // y*y+x*x;
198:    dd 47 d0            fld    u               // u; y*y+x*x;
19b:    d9 e1               fabs                   // |u|; y*y+x*x;
19d:    de c1               faddp  st(1),st        // y*y+x*x+|u|;
19f:    d9 fa               fsqrt                  // sqrt(y*y+x*x+|u|);
1a1:    dd 5d f8            fstp   t0              // t0 = sqrt(y * y + x * x + |u|)

1a4:    90                  nop

1a5:    dd 45 e8            fld    t2              // t2;
1a8:    d8 c8               fmul   st,st(0)        // t2*t2;
1aa:    dd 47 d8            fld    is              // is; t2*t2;
1ad:    de f1               fdivrp st(1),st        // is/(t2*t2);
1af:    dd 5d e8            fstp   t2              // t2 = is / (t2 * t2)

1b2:    90                  nop

1b3:    dd 01               fld    z               // z;
1b5:    dc 75 f8            fdiv   t0              // z/t0;
1b8:    d8 c8               fmul   st,st(0)        // (z*z)/(t0*t0);
1ba:    d9 e0               fchs                   // -(z*z)/(t0*t0);
1bc:    d9 e8               fld1                   // 1; -(z*z)/(t0*t0);
1be:    de c1               faddp  st(1),st        // -(z*z)/(t0*t0)+1;
1c0:    dd 5d f0            fstp   t1              // t1 = -(z * z) / (t0 * t0) + 1

1c3:    90                  nop

1c4:    dd 03               fld    x               // x;
1c6:    d8 c8               fmul   st,st(0)        // x*x;
1c8:    dd 02               fld    y               // y; x*x;
1ca:    d8 c8               fmul   st,st(0)        // y*y; x*x;
1cc:    d9 e0               fchs                   // -y*y; x*x;
1ce:    de c1               faddp  st(1),st        // x*x-y*y;
1d0:    dc 4d f0            fmul   t1              // (x*x-y*y)*t1;
1d3:    dc 4d e8            fmul   t2              // (x*x-y*y)*t1*t2;
1d6:    dc 46 18            fadd   cx              // (x*x-y*y)*t1*t2+cx;

1d9:    90                  nop

1da:    dd 03               fld    x               // x; (x*x-y*y)*t1*t2+cx;
1dc:    dc 0a               fmul   y               // x*y; (x*x-y*y)*t1*t2+cx;
1de:    d9 e0               fchs                   // -x*y; (x*x-y*y)*t1*t2+cx;
1e0:    dc 4d f0            fmul   t1              // -x*y*t1; (x*x-y*y)*t1*t2+cx;
1e3:    dc 4d e8            fmul   t2              // -x*y*t1*t2; (x*x-y*y)*t1*t2+cx;
1e6:    dc 46 20            fadd   cy              // -x*y*t1*t2+cy; (x*x-y*y)*t1*t2+cx;

1e9:    90                  nop

1ea:    dd 01               fld    z               // z; -x*y*t1*t2+cy; (x*x-y*y)*t1*t2+cx;
1ec:    d8 c0               fadd   st,st(0)        // 2*z; -x*y*t1*t2+cy; (x*x-y*y)*t1*t2+cx;
1ee:    d9 e0               fchs                   // -2*z; -x*y*t1*t2+cy; (x*x-y*y)*t1*t2+cx;
1f0:    dc 4d f8            fmul   t0              // -2*z*t0; -x*y*t1*t2+cy; (x*x-y*y)*t1*t2+cx;
1f3:    dc 4d e8            fmul   t2              // -2*z*t2; -x*y*t1*t2+cy; (x*x-y*y)*t1*t2+cx;
1f6:    dc 4f b0            fmul   izm             // -2*z*t0*izm; -x*y*t1*t2+cy; (x*x-y*y)*t1*t2+cx;
1f9:    dc 46 28            fadd   cz              // -2*z*t0*izm+cz; -x*y*t1*t2+cy; (x*x-y*y)*t1*t2+cx;

1fc:    90                  nop

1fd:    dd 19               fstp   z               // z = -2 * z * t0 * izm + cz, -y*t1*t2+cy; (x*x-y*y)*t1*t2+cx;
1ff:    dd 1a               fstp   y               // y = -x * y * t1 * t2 + cy, (x*x-y*y)*t1*t2+cx;
201:    dd 1b               fstp   x               // x = (x * x - y * y) * t1 * t2 + cx

203:    90                  nop
204:    90                  nop
205:    90                  nop
206:    90                  nop

207:    dd 03               fld    x               // x;
209:    dc 47 c8            fadd   ix              // x+ix;
20c:    dd 1b               fstp   x               // x = x + ix

20e:    dd 02               fld    y               // y;
210:    dc 47 c0            fadd   iy              // y+iy;
213:    dd 1a               fstp   y               // y = y + iy

215:    dd 01               fld    z               // z;
217:    dc 47 b8            fadd   iz              // z+iz;
21a:    dd 19               fstp   z               // z = z + iz

21c:    d9 d0               fnop

21e:    8b c3               mov    eax,ebx

220:    5f                  pop    edi
221:    5e                  pop    esi

222:    5b                  pop    ebx

223:    89 ec               mov    esp,ebp
225:    5d                  pop    ebp
226:    c2 08 00            ret    0x8
```

## Code

```rust
v.x = abs(v.x + f) - abs(v.x - f) + v.x;
v.y = abs(v.y + f) - abs(v.y - f) + v.y;
v.z = abs(v.z + f) - abs(v.z - f) + v.z;
let t2 = dot(v,v);
let t3 = length(v);
v.x = s * v.x - i.x;
v.y = s * v.y - i.y;
v.z = s * v.z - i.z;
if ((d == 0) && (t3 > 1.0)) {
    v.x += c.x;
    v.y += c.y;
    v.z += c.z;
}
else {
    let t0 = sqrt(v.y * v.y + v.x * v.x + abs(u));
    t2 = is / (t2 * t2);
    let t1 = -(v.z * v.z) / (t0 * t0) + 1;
    let q.x = (v.x * v.x - v.y * v.y) * t1 * t2;
    let q.y = -v.x * v.y * t2;
    let q.z = -2 * v.z * izm * t0;
    if (r >= t3) {
        v.x = q.x + c.x;
        v.y = q.y * t1 + c.y;
        v.z = q.z + c.z;
    }
    else {
        let t4 = (t3 - r) / (1 - r);
        let t5 = 1 - t4;
        v.x = t4 * v.x + q.x * t5;
        v.y = t4 * v.y + q.y * t5;
        v.z = t4 * v.z + q.z * t5 * t2;
    }
}
v.x += i.x;
v.y += i.y;
v.z += i.z;
```

## Vectorized

```rust
v = abs(v + vec3(f)) - abs(v - vec3(f)) + v;
let t2 = dot(v,v);
let t3 = length(v);
v = s * v - i;
if ((d == 0) && (t3 > 1.0)) {
    v += c;
}
else {
    let t0 = sqrt(v.y * v.y + v.x * v.x + abs(u));
    t2 = is / (t2 * t2);
    let t1 = -(v.z * v.z) / (t0 * t0) + 1;
    let q = vec3(
        (v.x * v.x - v.y * v.y) * t1 * t2,
        -v.x * v.y * t2,
        -2.0 * v.z * izm * t0,
    );
    if (r >= t3) {
        q.y *= t1;
        v = q + c;
    }
    else {
        let t4 = (t3 - r) / (1 - r);
        let t5 = 1 - t4;
        q.z *= t2;
        v = t4 * v + t5 * q;
    }
}
v += i;
```


    v.x = abs(v.x - AMAZINGSURF_FOLD) + v.x - abs(v.x + AMAZINGSURF_FOLD);
    v.y = abs(v.y - AMAZINGSURF_FOLD) + v.y - abs(v.y + AMAZINGSURF_FOLD);
