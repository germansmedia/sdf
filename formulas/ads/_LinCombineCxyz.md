# `_LinCombineCxyz`

```
[OPTIONS]
.Version = 2
.DEoption = -1
.Double CXx mul = 1
.Double CXy mul = 0
.Double CXz mul = 0
.Double CYx mul = 0
.Double CYy mul = 1
.Double CYz mul = 0
.Double CZx mul = 0
.Double CZy mul = 0
.Double CZz mul = 1
[CODE]
558BEC56578B75088B7E30DD4628DD4620DD4618D9C0DC4FF0D9C2DC4FE8DEC1
D9C3DC4FE0DEC1DD5E18D9C0DC4FD8D9C2DC4FD0DEC1D9C3DC4FC8DEC1DD5E20
DC4FC0D9C9DC4FB8DEC1D9C9DC4FB0DEC1DD5E285F5E5DC20800
[END]

Description:

A general 3x3 matrix conversion for C:

Cx' = CXx*Cx + CXy*Cy + CXz*Cz
Cy' = CYx*Cx + CYy*Cy + CYz*Cz
Cz' = CZx*Cx + CZy*Cy + CZz*Cz


You can use it for example as a pretransform for bulbs or boxes.
```

## Disassembly

[edi-0x10] = mat.x.x
[edi-0x18] = mat.x.y
[edi-0x20] = mat.x.z
[edi-0x28] = mat.y.x
[edi-0x30] = mat.y.y
[edi-0x38] = mat.y.z
[edi-0x40] = mat.z.x
[edi-0x48] = mat.z.y
[edi-0x50] = mat.z.z

```
0:  55                      push   ebp
1:  8b ec                   mov    ebp,esp

3:  56                      push   esi
4:  57                      push   edi

5:  8b 75 08                mov    esi,DWORD PTR [ebp+0x8]
8:  8b 7e 30                mov    edi,DWORD PTR [esi+0x30]

b:  dd 46 28                fld    c.z         // c.z;
e:  dd 46 20                fld    c.y         // c.y; c.z;
11: dd 46 18                fld    c.x         // c.x; c.y; c.z;
14: d9 c0                   fld    st(0)       // c.x; c.x; c.y; c.z;
16: dc 4f f0                fmul   mat.x.x     // c.x * mat.x.x; c.x; c.y; c.z;
19: d9 c2                   fld    st(2)       // c.y; c.x * mat.x.x; c.x; c.y; c.z;
1b: dc 4f e8                fmul   mat.x.y     // c.y * mat.x.y; c.x * mat.x.x; c.x; c.y; c.z;
1e: de c1                   faddp  st(1),st    // c.x * mat.x.x + c.y * mat.x.y; c.x; c.y; c.z;
20: d9 c3                   fld    st(3)       // c.z; c.x * mat.x.x + c.y * mat.x.y; c.x; c.y; c.z;
22: dc 4f e0                fmul   mat.x.z     // c.z * mat.x.z; c.x * mat.x.x + c.y * mat.x.y; c.x; c.y; c.z;
25: de c1                   faddp  st(1),st    // dot(c,mat.x); c.x; c.y; c.z;
27: dd 5e 18                fstp   c.x         // c.x = dot(c,mat.x), c.x; c.y; c.z;

2a: d9 c0                   fld    st(0)       // c.x; c.x; c.y; c.z;
2c: dc 4f d8                fmul   mat.y.x     // c.x * mat.y.x; c.x; c.y; c.z;
2f: d9 c2                   fld    st(2)       // c.y; c.x * mat.y.x; c.x; c.y; c.z;
31: dc 4f d0                fmul   mat.y.y     // c.y * mat.y.y; c.x * mat.y.x; c.x; c.y; c.z;
34: de c1                   faddp  st(1),st    // c.x * mat.y.x + c.y * mat.y.y; c.x; c.y; c.z;
36: d9 c3                   fld    st(3)       // c.z; c.x * mat.y.x + c.y * mat.y.y; c.x; c.y; c.z;
38: dc 4f c8                fmul   mat.y.z     // c.z * mat.y.z; c.x * mat.y.x + c.y * mat.y.y; c.x; c.y; c.z;
3b: de c1                   faddp  st(1),st    // dot(c,mat.y); c.x; c.y; c.z;
3d: dd 5e 20                fstp   c.y         // c.y = dot(c,mat.y), c.x; c.y; c.z;

40: dc 4f c0                fmul   mat.z.x     // c.x * mat.z.x; c.y; c.z;
43: d9 c9                   fxch   st(1)       // c.y; c.x * mat.z.x; c.z;
45: dc 4f b8                fmul   mat.z.y     // c.y * mat.z.y; c.x * mat.z.x; c.z;
48: de c1                   faddp  st(1),st    // c.x * mat.z.x + c.y * mat.z.y; c.z;
4a: d9 c9                   fxch   st(1)       // c.z; c.x * mat.z.x + c.y * mat.z.y;
4c: dc 4f b0                fmul   mat.z.z     // c.z * mat.z.z; c.x * mat.z.x + c.y * mat.z.y;
4f: de c1                   faddp  st(1),st    // dot(c,mat.z);
51: dd 5e 28                fstp   c.z         // c.z = dot(c,mat.z);

54: 5f                      pop    edi
55: 5e                      pop    esi

56: 5d                      pop    ebp
57: c2 08 00                ret    0x8
```

## Code

```glsl
c = mat * c;
```
