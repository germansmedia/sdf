# `_HopSqrtX`

```
[OPTIONS]
.Version = 2
.DEoption = -1
.Double fixX = 0.0
.Double fixSq = 0.0
.Double Mul = 1.0
.Double Div = 1.0
[CODE]
558BEC56538B75088BD88B7630DD03DC4EE0DC46F0D9C0D9E1D9FAD9C9D9EEDE
D9DFE080E4417F02D9E0DC76D8DC46E8DD1B8BC35B5E5DC20800
[END]

Description:

Sqrt for x as defined in Hopalong attractor + some bells & whistles

Formula by Luca GN 2011

t =  Mul * x + fixX 
if t>0
x' = fixSq + sqrt abs ( t )/Div
else
x' = fixSq - sqrt abs ( t )/Div 
endif
```

## Disassembly

[ebx] = v.x
[esi-0x10] = fixx
[esi-0x18] = fixsq
[esi-0x20] = mul
[esi-0x28] = div

```
0:  55                      push   ebp
1:  8b ec                   mov    ebp,esp

3:  56                      push   esi
4:  53                      push   ebx

5:  8b 75 08                mov    esi,DWORD PTR [ebp+0x8]
8:  8b d8                   mov    ebx,eax
a:  8b 76 30                mov    esi,DWORD PTR [esi+0x30]

d:  dd 03                   fld    v.x         // v.x;
f:  dc 4e e0                fmul   mul         // v.x * mul;
12: dc 46 f0                fadd   fixx        // v.x * mul + fixx;
15: d9 c0                   fld    st(0)       // v.x * mul + fixx; v.x * mul + fixx;
17: d9 e1                   fabs               // |v.x * mul + fixx|; v.x * mul + fixx;
19: d9 fa                   fsqrt              // sqrt(|v.x * mul + fixx|); v.x * mul + fixx;
1b: d9 c9                   fxch   st(1)       // v.x * mul + fixx; sqrt(|v.x * mul + fixx|);
1d: d9 ee                   fldz               // 0.0; v.x * mul + fixx; sqrt(|v.x * mul + fixx|);
1f: de d9                   fcompp             // 0.0 ? v.x * mul + fixx, sqrt(|v.x * mul + fixx|);
21: df e0                   fnstsw ax
23: 80 e4 41                and    ah,0x41
26: 7f 02                   jg     0x2a

28: d9 e0                   fchs               // +/- sqrt(|v.x * mul + fixx|);

2a: dc 76 d8                fdiv   div         // +/- sqrt(|v.x * mul + fixx|) / div;
2d: dc 46 e8                fadd   fixsq       // fixsq +/- sqrt(|v.x * mul + fixx|) / div;
30: dd 1b                   fstp   v.x         // v.x = fixsq +/- sqrt(|v.x * mul + fixx|) / div

32: 8b c3                   mov    eax,ebx

34: 5b                      pop    ebx
35: 5e                      pop    esi

36: 5d                      pop    ebp
37: c2 08 00                ret    0x8
```

## Code

```glsl
q = mul * v.x + fixx;
r = sqrt(abs(q));
if (q < 0) {
    r = -r;
}
v.x = r / div + fixsq;
```
