# `koch_cube`

```
[OPTIONS]
.Version = 5
.DEoption = 2
.DEscale = 1
.SIpower = 1
.RStop = 5
.Double Post-scale = 1
.Double XY Stretch = 1
.Double Z Fold = 1
.Double X Add = 0
.Double Y Add = 0
.Double Z Add = 0
.3SingleAngles Rotation = 0
[CONSTANTS]
Double = 2
Double = 3
Double = 4
[CODE]
558BEC81EC200000005356578B75088B
7E308BD8DD03D9E1DC4F08DD1B90DD02
D9E1DC4F08DD1A90DD01D9E1DC4F08DD
19DD4108DC4F08DC77E8DC4FF0DD5908
90D9D0DD03DD02DED9DFE080E4417708
DD03DD02DD1BDD1ADD03DD01DED9DFE0
80E4417708DD03DD01DD1BDD19DD01DD
02DED9DFE080E4417608DD01DD02DD19
DD1AD9D0DD01DC47C8DD02DC47D0DD03
DC47D8D9C0D84FACD9C2D84FA8DEC1D9
C3D84FA4DEC1D9C1D84FC4D9C3D84FC0
DEC1D9C4D84FBCDEC1D9CAD84FB8D9CB
D84FB4DEC3D9CBD84FB0DEC2DD1BDD1A
DD1990DD47E0DC21D9E1D9E0DC47E0DD
1990DD4708D9C0DD47E8DEC1DD5DE8DD
47E8DEE9DD5DE090DD03DC65E0DD5DF8
90DD03DC65E8DD5DF090DD45F8DD02DE
D9DFE080E4417712DD45F8DD1BDD02DC
65E0DD1AEB2690909090DD45F0DD02DE
D9DFE080E441760990DD45F0DD1BEB0C
90DD02DD45F0DD1ADD1BD9D0DD03DC4F
F0DC77E8DD1BDD02DC4FF0DC77E8DD1A
DD01DC4FF0DD198BC35F5E5B89EC5DC2
0800
[END]
Cubic Koch fractal. Inspired from a Wikipedia image ( http://en.wikipedia.org/wiki/File:Quadratic_Koch_3D_(type1_stage2).png )
Added options for scale, stretching and other goodies
TIP - With default settings you don't see the fractal correctly; decrease DEstop, and use miniter=10 or something to see a better
plot. Be careful with scale and stretching tweaks, they greatly affect the render!
If you use xy stretch, the fractal will be less uniform but with some singularities.

----------------
  ; basic cubic fold (from Menger3 formula)
  ; scale has been replaced by a postscale 
  ; to fix some branch cut troubles
  x = abs(x)*3
  y = abs(y)*3
  z = abs(z)*3
  if y>x
  t=x
  x=y
  y=t
  endif
  if z>x
  t=x
  x=z
  z=t
  endif
  if z>y
  t=y
  y=z
  z=t
  endif
  x = x+xa, y=y+ya, z=z+za
  ( ... rotation of x,y,z on specified angles ... )
  ; basic vars
  a=3-stretch, b=3+stretch, c=x-a, d=x-b
  ; the routine works only for scale = 3, else some discontinuities will appear
  ; Now, draw the cube WITHOUT making discontinuites in the spatial vars
  ; bring what falls "out of the small cube zone" in the interval (-1;1) (next iter will take care)
  ; z must be folded to get a homogen look
  z=zfix-abs(zfix-z)
  if (c<y)
   ; out of the cube ... normalize
   x=c
   y=y-a ; NOT a-y -> discontinue
  elseif (d>y)
   ; point falls just over the small cube
   x=d
  else
   ; vertical faces of small cube (swap)
   t=x
   x=y
   y=d
  endif
  x = x/stretch, y=y/stretch ;NON conformal stretching
  x = x*postscale, y = y*postscale, z = z*postscale
----------------
FORMULA AND IMPLEMENTATION; 100% by Luca G.N. 2011.
```

## Disassembly

```
[ebx] = x
[edx] = y
[ecx] = z
[ecx+08] = w
esi = TIteration3Dext
edi = PVar
[edi-5C] = rot.z.z
[edi-58] = rot.z.y
[edi-54] = rot.z.x
[edi-50] = rot.y.z
[edi-4C] = rot.y.y
[edi-48] = rot.y.x
[edi-44] = rot.x.z
[edi-40] = rot.x.y
[edi-3C] = rot.x.x
[edi-38] = Z Add
[edi-30] = Y Add
[edi-28] = X Add
[edi-20] = Z Fold
[edi-18] = XY Stretch
[edi-10] = Post-scale
[edi-08] = 0.5
[edi] = 2
[edi+08] = 3
[edi+10] = 4
[ebp-08] = d
[ebp-10] = c
[ebp-18] = b
[ebp-20] = a

0:  55                      push   ebp
1:  8b ec                   mov    ebp,esp
3:  81 ec 20 00 00 00       sub    esp,0x20             // reserve space for a, b, c and d
9:  53                      push   ebx
a:  56                      push   esi
b:  57                      push   edi
c:  8b 75 08                mov    esi,TIteration3Dext
f:  8b 7e 30                mov    edi,PVar
12: 8b d8                   mov    ebx,eax

v = 3 * abs(v)

14: dd 03                   fld    x                    // x
16: d9 e1                   fabs                        // abs(x)
18: dc 4f 08                fmul   3                    // 3 * abs(x)
1b: dd 1b                   fstp   x                    // x = 3 * abs(x)
1d: 90                      nop
1e: dd 02                   fld    y                    // y
20: d9 e1                   fabs                        // abs(y)
22: dc 4f 08                fmul   3                    // 3 * abs(y)
25: dd 1a                   fstp   y                    // y = 3 * abs(y)
27: 90                      nop
28: dd 01                   fld    z                    // z
2a: d9 e1                   fabs                        // abs(z)
2c: dc 4f 08                fmul   3                    // 3 * abs(z)
2f: dd 19                   fstp   z                    // z = 3 * abs(z)

dr = 3 * post_scale * dr / xy_stretch

31: dd 41 08                fld    w                    // w
34: dc 4f 08                fmul   3                    // 3 * w
37: dc 77 e8                fdiv   XY Stretch           // 3 * w / xy_stretch
3a: dc 4f f0                fmul   Post-scale           // 3 * post_scale * w / xy_stretch
3d: dd 59 08                fstp   w                    // w = 3 * post_scale * w / xy_stretch

40: 90                      nop
41: d9 d0                   fnop

// sort components so that v.x is the largest and v.z is the smallest

if (v.y > v.x) {
    swap(v.x,v.y);
}

43: dd 03                   fld    x                    // x
45: dd 02                   fld    y                    // y; x
47: de d9                   fcompp                      // y ? x (C3 = y == x, C0 = y < x)
49: df e0                   fnstsw ax
4b: 80 e4 41                and    ah,0x41              // ZF = (y != x) && (y >= x)
4e: 77 08                   ja     0x58                 // y <= x

50: dd 03                   fld    x                    // x
52: dd 02                   fld    y                    // y; x
54: dd 1b                   fstp   x                    // x = y; x
56: dd 1a                   fstp   y                    // y = what x was at 50

if (v.z > v.x) {
    swap(v.x,v.z);
}

58: dd 03                   fld    x                    // x
5a: dd 01                   fld    z                    // z; x
5c: de d9                   fcompp                      // z ? x (C3 = z == x, C0 = z < x)
5e: df e0                   fnstsw ax
60: 80 e4 41                and    ah,0x41              // ZF = (z != x) && (z >= x)
63: 77 08                   ja     0x6d                 // z <= x

65: dd 03                   fld    x                    // x
67: dd 01                   fld    z                    // z; x
69: dd 1b                   fstp   x                    // x = z; x
6b: dd 19                   fstp   z                    // z = what x was at 65

if (v.y > v.z) {
    swap(v.y,v.z);
}

6d: dd 01                   fld    z                    // z
6f: dd 02                   fld    y                    // y; z
71: de d9                   fcompp                      // y ? z (C3 = y == z, C0 = y < z)
73: df e0                   fnstsw ax
75: 80 e4 41                and    ah,0x41              // ZF = (y != z) && (y >= z)
78: 76 08                   jna    0x82                 // y > z

7a: dd 01                   fld    z                    // z
7c: dd 02                   fld    y                    // y; z
7e: dd 19                   fstp   z                    // z = y; z
80: dd 1a                   fstp   y                    // y = what z was at 7a

82: d9 d0                   fnop

v = rot * (v + v_add)

84: dd 01                   fld    z                    // z
86: dc 47 c8                fadd   Z Add                // z + z_add
89: dd 02                   fld    y                    // y; z + z_add
8b: dc 47 d0                fadd   Y Add                // y + y_add; z + z_add
8e: dd 03                   fld    x                    // x; y + y_add; z + z_add
90: dc 47 d8                fadd   X Add                // x + x_add; y + y_add; z + z_add
93: d9 c0                   fld    st(0)
95: d8 4f ac                fmul   rot.z.x
98: d9 c2                   fld    st(2)
9a: d8 4f a8                fmul   rot.z.y
9d: de c1                   faddp  st(1),st
9f: d9 c3                   fld    st(3)
a1: d8 4f a4                fmul   rot.z.z
a4: de c1                   faddp  st(1),st
a6: d9 c1                   fld    st(1)
a8: d8 4f c4                fmul   rot.x.x
ab: d9 c3                   fld    st(3)
ad: d8 4f c0                fmul   rot.x.y
b0: de c1                   faddp  st(1),st
b2: d9 c4                   fld    st(4)
b4: d8 4f bc                fmul   rot.x.z
b7: de c1                   faddp  st(1),st
b9: d9 ca                   fxch   st(2)
bb: d8 4f b8                fmul   rot.y.x
be: d9 cb                   fxch   st(3)
c0: d8 4f b4                fmul   rot.y.y
c3: de c3                   faddp  st(3),st
c5: d9 cb                   fxch   st(3)
c7: d8 4f b0                fmul   rot.y.z
ca: de c2                   faddp  st(2),st
cc: dd 1b                   fstp   x
ce: dd 1a                   fstp   y
d0: dd 19                   fstp   z

d2: 90                      nop

v.z = z_fold - abs(z_fold - v.z)

d3: dd 47 e0                fld    Z Fold               // z_fold
d6: dc 21                   fsub   z                    // z_fold - z
d8: d9 e1                   fabs                        // abs(z_fold - z)
da: d9 e0                   fchs                        // -abs(z_fold - z)
dc: dc 47 e0                fadd   Z Fold               // -abs(z_fold - z) + z_fold
df: dd 19                   fstp   z                    // z = z_fold - abs(z_fold - z)

e1: 90                      nop

a = xy_stretch - 3
b = xy_stretch + 3

e2: dd 47 08                fld    3                    // 3
e5: d9 c0                   fld    st(0)                // 3; 3
e7: dd 47 e8                fld    XY Stretch           // xy_stretch; 3; 3
ea: de c1                   faddp  st(1),st             // xy_stretch + 3; 3
ec: dd 5d e8                fstp   b                    // b = xy_stretch + 3; 3
ef: dd 47 e8                fld    XY Stretch           // xy_stretch; 3
f2: de e9                   fsubp  st(1),st             // xy_stretch - 3
f4: dd 5d e0                fstp   a                    // a = xy_stretch - 3

f7: 90                      nop

d = x - a

f8: dd 03                   fld    x                    // x
fa: dc 65 e0                fsub   a                    // x - a
fd: dd 5d f8                fstp   c                    // c = x - a

100:    90                      nop

c = x - b

101:    dd 03                   fld    x                // x
103:    dc 65 e8                fsub   b                // x - b
106:    dd 5d f0                fstp   d                // d = x - b

109:    90                      nop

if (y > c) {
    x = c;
    y = y - a;
}

10a:    dd 45 f8                fld    c                // c
10d:    dd 02                   fld    y                // y; c
10f:    de d9                   fcompp                  // y ? c (C3 = y == c, C0 = y < c)
111:    df e0                   fnstsw ax
113:    80 e4 41                and    ah,0x41          // ZF = (y != c) && (y >= c)
116:    77 12                   ja     0x12a            // y <= c

118:    dd 45 f8                fld    c                // c
11b:    dd 1b                   fstp   x                // x = c
11d:    dd 02                   fld    y                // y
11f:    dc 65 e0                fsub   a                // y - a
122:    dd 1a                   fstp   y                // y = y - a
124:    eb 26                   jmp    0x14c

126:    90                      nop
127:    90                      nop
128:    90                      nop
129:    90                      nop

else if (y <= d) {
    x = d;
}

12a:    dd 45 f0                fld    d                // d
12d:    dd 02                   fld    y                // y; d
12f:    de d9                   fcompp                  // y ? d (C3 = y == d, C0 = y < d)
131:    df e0                   fnstsw ax
133:    80 e4 41                and    ah,0x41          // ZF = (y != d) && (y >= d)
136:    76 09                   jna    0x141            // y > d

138:    90                      nop
139:    dd 45 f0                fld    d                // d
13c:    dd 1b                   fstp   x                // x = d
13e:    eb 0c                   jmp    0x14c
140:    90                      nop

else {
    x = y;
    y = d;
}

141:    dd 02                   fld    y                // y
143:    dd 45 f0                fld    d                // d; y
146:    dd 1a                   fstp   y                // y = d; y
148:    dd 1b                   fstp   x                // x = what y was at 141
14a:    d9 d0                   fnop

v = post_scale * vec3(v.x / xy_stretch,v.y / xy_stretch,v.z)

14c:    dd 03                   fld    x                // x
14e:    dc 4f f0                fmul   Post-scale       // post_scale * x
151:    dc 77 e8                fdiv   XY Stretch       // post_scale * x / xy_stretch
154:    dd 1b                   fstp   x                // x = post_scale * x / xy_stretch
156:    dd 02                   fld    y                // y
158:    dc 4f f0                fmul   Post-scale       // post_scale * y
15b:    dc 77 e8                fdiv   XY Stretch       // post_scale * y / xy_stretch
15e:    dd 1a                   fstp   y                // y = post_scale * y / xy_stretch
160:    dd 01                   fld    z                // z
162:    dc 4f f0                fmul   Post-scale       // post_scale * z
165:    dd 19                   fstp   z                // z = post_scale * z

167:    8b c3                   mov    eax,ebx
169:    5f                      pop    edi
16a:    5e                      pop    esi
16b:    5b                      pop    ebx
16c:    89 ec                   mov    esp,ebp
16e:    5d                      pop    ebp
16f:    c2 08 00                ret    0x8
```

## Code

```glsl
v = 3 * abs(v);
dr = 3 * post_scale * dr / xy_stretch;

if (v.y <= v.x) {
    swap(v.x,v.y);
}
if (v.z <= v.x) {
    swap(v.x,v.z);
}
if (v.y >= v.z) {
    swap(v.y,v.z);
}

v = rot * (v + v_add);

v.z = z_fold - abs(z_fold - v.z);

a = xy_stretch - 3;
b = xy_stretch + 3;
d = x - a;
c = x - b;

if (y <= d) {
    x = d;
    y = y - a;
}
else if (y > c) {
    x = c;
}
else {
    x = y;
    y = c;
}

v = post_scale * vec3(v.x / xy_stretch,v.y / xy_stretch,v.z);
```