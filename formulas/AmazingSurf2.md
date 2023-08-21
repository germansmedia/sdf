# AmazingSurf2

```
[OPTIONS]
.Version = 4
.DEscale = .2
.DEoption = 11
.RStop = 20
.SIpower = 2
.Double Scale = 1.5
.Boxscale Min R = 0.5
.Double Fold = 1
.3SingleAngles Rotation1 = 5
.Double Scale vary = 0
.Integer Sphere or Cylinder = 1
[CODE]
558BEC5657538B7D0889C38B773081C780000000837F50007F09FF4750DD46F0
DD5F48DD4748D9E8D9C1D9E1DEE1DC4EACDEC1DD5F48DD46D8DD03D9C0D8E2D9
E1D8C1D9C9D8C2D9E1DEE1DD02D9C0D8E3D9E1D8C1D9C9D8C3D9E1DEE1DD12D8
C8D9C9DD13D8C8DEC1DDD9837EA8007506DD01D8C8DEC1DC56E0DFE0D0EC7307
DDD8DD46E8EB16D9E8D8D1DFE0D0EC7207DDD8DC7F48EB05DED9DD4748DD4108
D8C9DD5908DD01D8C9DC47A8DD02D8CADC47A0DD03D8CBDC4798D9C0D84EBCD9
C2D84EB8DEC1D9C3D84EB4DEC1D9C1D84ED4D9C3D84ED0DEC1D9C4D84ECCDEC1
D9CAD84EC8D9CBD84EC4DEC3D9CBD84EC0DEC2DD1BDD1ADD19DDD89090909089
D85B5F5E5DC20800
[END]


Description:

Formula proposed by Kali by a private message. I added some features btw
Luca GN 2012

Scale = Scale + Scale_vary*(abs(Scale)-1)
x = TgladFold(x,fold)
y = TgladFold(y,fold)
// z is not folded
if SorC != 0
rr = x*x + y*y // cylinder shape for inversion (sometimes buggy but can be cool)
else
rr = x*x + y*y + z*z // sphere shape for inversion (works better)
endif
if rr < sqr(Min_R) then m = Scale/sqr(Min_R) else
if rr < 1 then m = Scale/rr else m = Scale
x = x * m + Cy
y = y * m + Cx
z = z * m + Cz
Rotate3D(x,y,z,angles)
```

## Disassembly

[eax] = x
[edx] = y
[ecx] = z
[ecx+0x08] = w
[esi-0x08] = 0.5
[esi-0x10] = s
[esi-0x18] = s / (r * r)
[esi-0x20] = r * r
[esi-0x28] = f
[esi-0x2c] = r0
[esi-0x30] = r1
[esi-0x34] = r2
[esi-0x38] = r3
[esi-0x3c] = r4
[esi-0x40] = r5
[esi-0x44] = r6
[esi-0x48] = r8
[esi-0x4c] = r9
[esi-0x54] = vary
[esi-0x58] = sphere_or_cylinder

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

14: 83 7f 50 00             cmp    some_int,0x0
18: 7f 09                   jg     0x23          // some_int > 0

// some_int <= 0:
1a: ff 47 50                inc    some_int      // some_int += 1
1d: dd 46 f0                fld    s             // s;
20: dd 5f 48                fstp   q             // q = s

// some_int > 0:
23: dd 47 48                fld    q             // q;
26: d9 e8                   fld1                 // 1; q;
28: d9 c1                   fld    st(1)         // q; 1; q;
2a: d9 e1                   fabs                 // |q|; 1; q;
2c: de e1                   fsubrp st(1),st      // |q|-1; q;
2e: dc 4e ac                fmul   vary          // vary*(|q|-1); q;
31: de c1                   faddp  st(1),st      // vary*(|q|-1)+q;
33: dd 5f 48                fstp   q             // q = vary * (|q| - 1) + q

36: dd 46 d8                fld    f             // f;
39: dd 03                   fld    x             // x; f;
3b: d9 c0                   fld    st(0)         // x; x; f;
3d: d8 e2                   fsub   st,st(2)      // x-f; x; f;
3f: d9 e1                   fabs                 // |x-f|; x; f;
41: d8 c1                   fadd   st,st(1)      // |x-f|+x; x; f;
43: d9 c9                   fxch   st(1)         // x; |x-f|+x; f;
45: d8 c2                   fadd   st,st(2)      // x+f; |x-f|+x; f;
47: d9 e1                   fabs                 // |x+f|; |x-f|+x; f;
49: de e1                   fsubrp st(1),st      // |x+f|-(|x-f|+x); f;
4b: dd 02                   fld    y             // y; |x+f|-(|x-f|+x); f;
4d: d9 c0                   fld    st(0)         // y; y; |x+f|-(|x-f|+x); f;
4f: d8 e3                   fsub   st,st(3)      // y-f; y; |x+f|-(|x-f|+x); f;
51: d9 e1                   fabs                 // |y-f|; y; |x+f|-(|x-f|+x); f;
53: d8 c1                   fadd   st,st(1)      // |y-f|+y; y; |x+f|-(|x-f|+x); f;
55: d9 c9                   fxch   st(1)         // y; |y-f|+y; |x+f|-(|x-f|+x); f;
57: d8 c3                   fadd   st,st(3)      // y+f; |y-f|+y; |x+f|-(|x-f|+x); f;
59: d9 e1                   fabs                 // |y+f|; |y-f|+y; |x+f|-(|x-f|+x); f;
5b: de e1                   fsubrp st(1),st      // |y+f|-(|y-f|+y); |x+f|-(|x-f|+x); f;
5d: dd 12                   fst    y             // y = |y + f| - (|y - f| + y), |y+f|-(|y-f|+y); |x+f|-(|x-f|+x); f;
5f: d8 c8                   fmul   st,st(0)      // (|y+f|-(|y-f|+y))^2; |x+f|-(|x-f|+x); f;
61: d9 c9                   fxch   st(1)         // |x+f|-(|x-f|+x); (|y+f|-(|y-f|+y))^2; f;
63: dd 13                   fst    x             // x = |x + f| - (|x - f| + x), |x+f|-(|x-f|+x); (|y+f|-(|y-f|+y))^2; f;
65: d8 c8                   fmul   st,st(0)      // (|x+f|-(|x-f|+x))^2; (|y+f|-(|y-f|+y))^2; f;
67: de c1                   faddp  st(1),st      // (|y+f|-(|y-f|+y))^2+(|x+f|-(|x-f|+x))^2; f;
69: dd d9                   fstp   st(1)         // (|y+f|-(|y-f|+y))^2+(|x+f|-(|x-f|+x))^2;

6b: 83 7e a8 00             cmp    soc,0x0       // soc != 0x0
6f: 75 06                   jne    0x77

// soc == 0:
71: dd 01                   fld    z             // z; (|y+f|-(|y-f|+y))^2+(|x+f|-(|x-f|+x))^2;
73: d8 c8                   fmul   st,st(0)      // z^2; (|y+f|-(|y-f|+y))^2+(|x+f|-(|x-f|+x))^2;
75: de c1                   faddp  st(1),st      // (|y+f|-(|y-f|+y))^2+(|x+f|-(|x-f|+x))^2+z^2;

// soc != 0:
77: dc 56 e0                fcom   r * r         // st ? r*r
7a: df e0                   fnstsw ax
7c: d0 ec                   shr    ah,1
7e: 73 07                   jae    0x87

80: dd d8                   fstp   st(0)         //
82: dd 46 e8                fld    s / (r * r)   // s/(r*r)
85: eb 16                   jmp    0x9d

87: d9 e8                   fld1                 // 1; st1;
89: d8 d1                   fcom   st(1)         // 1 ? st1
8b: df e0                   fnstsw ax
8d: d0 ec                   shr    ah,1
8f: 72 07                   jb     0x98

91: dd d8                   fstp   st(0)         // st1;
93: dc 7f 48                fdivr  q             // q/st1;
96: eb 05                   jmp    0x9d

98: de d9                   fcompp               //
9a: dd 47 48                fld    q             // q;

9d: dd 41 08                fld    w             // w; q;
a0: d8 c9                   fmul   st,st(1)      // w*q; q;
a2: dd 59 08                fstp   w             // w = w * q, q;
a5: dd 01                   fld    z             // z; q;
a7: d8 c9                   fmul   st,st(1)      // z*q; q;
a9: dc 47 a8                fadd   cz            // z*q+cz; q;
ac: dd 02                   fld    y             // y; z*q+cz; q;
ae: d8 ca                   fmul   st,st(2)      // y*q; z*q+cz; q;
b0: dc 47 a0                fadd   cy            // y*q+cy; z*q+cz; q;
b3: dd 03                   fld    x             // x; y*q+cy; z*q+cz; q;
b5: d8 cb                   fmul   st,st(3)      // x*q; y*q+cy; z*q+cz; q;
b7: dc 47 98                fadd   cx            // x*q+cx; y*q+cy; z*q+cz; q;

// rotate
ba: d9 c0                   fld    st(0)
bc: d8 4e bc                fmul   r6
bf: d9 c2                   fld    st(2)
c1: d8 4e b8                fmul   r7
c4: de c1                   faddp  st(1),st
c6: d9 c3                   fld    st(3)
c8: d8 4e b4                fmul   r8
cb: de c1                   faddp  st(1),st
cd: d9 c1                   fld    st(1)
cf: d8 4e d4                fmul   r0
d2: d9 c3                   fld    st(3)
d4: d8 4e d0                fmul   r1
d7: de c1                   faddp  st(1),st
d9: d9 c4                   fld    st(4)
db: d8 4e cc                fmul   r2
de: de c1                   faddp  st(1),st
e0: d9 ca                   fxch   st(2)
e2: d8 4e c8                fmul   r3
e5: d9 cb                   fxch   st(3)
e7: d8 4e c4                fmul   r4
ea: de c3                   faddp  st(3),st
ec: d9 cb                   fxch   st(3)
ee: d8 4e c0                fmul   r5
f1: de c2                   faddp  st(2),st

f3: dd 1b                   fstp   x
f5: dd 1a                   fstp   y
f7: dd 19                   fstp   z

f9: dd d8                   fstp   st(0)

fb: 90                      nop
fc: 90                      nop
fd: 90                      nop
fe: 90                      nop

ff: 89 d8                   mov    eax,ebx
101:    5b                      pop    ebx

102:    5f                      pop    edi
103:    5e                      pop    esi

104:    5d                      pop    ebp
105:    c2 08 00                ret    0x8
```

## Code

```rust
let q = vary * (|scale| - 1) + scale;
x = |x + f| - (|x - f| + x);
y = |y + f| - (|y - f| + y);
let rr = x * x + y * y;
if sphere {
    rr += z * z;
}
let q = if rr < minr * minr {
    q / (minr * minr)
}
else if rr < 1 {
    q / rr
}
else {
    q
};
dr *= q;
x = q * x + cx;
y = q * y + cy;
z = q * z + cz;
rotate
```
