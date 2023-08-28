# `_Rotate4d`

```
[OPTIONS]
.Version = 5
.DEoption = -1
.6SingleAngles Rotation = 0
[CODE]
558BEC568B75088B7630DD4108DD01DD02DD00D9C0D84EF4D9C2D84EF0DEC1D9
C3D84EECDEC1D9C4D84EE8DEC1DD18D9C0D84EE4D9C2D84EE0DEC1D9C3D84EDC
DEC1D9C4D84ED8DEC1DD1AD9C0D84ED4D9C2D84ED0DEC1D9C3D84ECCDEC1D9C4
D84EC8DEC1DD19D84EC4D9C9D84EC0DEC1D9C9D84EBCDEC1D9C9D84EB8DEC1DD
59085E5DC20800
[END]
```

## Disassembly

`[EAX]` = x
`[EDX]` = y
`[ECX]` = z
`[ECX+08]` = w
`[ESI-08]` = 0.5 (f64)
`[ESI-0C]..[ESI-4C]` = Rotation (Mat4x4<f32>)

```
00000000 55                              PUSH EBP
00000001 8BEC                            MOV EBP,ESP

00000003 56                              PUSH ESI

00000004 8B7508                          MOV ESI,DWORD PTR [EBP+08]
00000007 8B7630                          MOV ESI,DWORD PTR [ESI+30]

0000000A DD4108                          FLD w           // w;
0000000D DD01                            FLD z           // z; w;
0000000F DD02                            FLD y           // y; z; w;
00000011 DD00                            FLD x           // x; y; z; w;
00000013 D9C0                            FLD x           // x; x; y; z; w;
00000015 D84EF4                          FMUL r0         // r0 x; x; y; z; w;
00000018 D9C2                            FLD ST(2)       // y; r0 x; x; y; z; w;
0000001A D84EF0                          FMUL r1         // r1 y; r0 x; x; y; z; w;
0000001D DEC1                            FADDP ST(1),ST  // r0 x + r1 y; x; y; z; w;
0000001F D9C3                            FLD ST(3)       // z; r0 x + r1 y; x; y; z; w;
00000021 D84EEC                          FMUL r2         // r2 z; r0 x + r1 y; x; y; z; w;
00000024 DEC1                            FADDP ST(1),ST  // r0 x + r1 y + r2 z; x; y; z; w;
00000026 D9C4                            FLD ST(4)       // w; r0 x + r1 y + r2 z; x; y; z; w;
00000028 D84EE8                          FMUL r3         // r3 w; r0 x + r1 y + r2 z; x; y; z; w;
0000002B DEC1                            FADDP ST(1),ST  // r0 x + r1 y + r2 z + r3 w; x; y; z; w;
0000002D DD18                            FSTP x          // x = r0 x + r1 y + r2 z + r3 w, x; y; z; w;

0000002F D9C0                            FLD ST          // x; x; y; z; w;
00000031 D84EE4                          FMUL r4         // r4 x; x; y; z; w;
00000034 D9C2                            FLD ST(2)       // y; r4 x; x; y; z; w;
00000036 D84EE0                          FMUL r5         // r5 y; r4 x; x; y; z; w;
00000039 DEC1                            FADDP ST(1),ST  // r4 x + r5 y; x; y; z; w;
0000003B D9C3                            FLD ST(3)       // z; r4 x + r5 y; x; y; z; w;
0000003D D84EDC                          FMUL r6         // r6 z; r4 x + r5 y; x; y; z; w;
00000040 DEC1                            FADDP ST(1),ST  // r4 x + r5 y + r6 z; x; y; z; w;
00000042 D9C4                            FLD ST(4)       // w; r4 x + r5 y + r6 z; x; y; z; w;
00000044 D84ED8                          FMUL r7         // r7 w; r4 x + r5 y + r6 z; x; y; z; w;
00000047 DEC1                            FADDP ST(1),ST  // r4 x + r5 y + r6 z + r7 w; x; y; z; w;
00000049 DD1A                            FSTP y          // y = r4 x + r5 y + r6 z + r7 w, x; y; z; w;

0000004B D9C0                            FLD ST          // x; x; y; z; w;
0000004D D84ED4                          FMUL r8         // r8 x; x; y; z; w;
00000050 D9C2                            FLD ST(2)       // y; r8 x; x; y; z; w;
00000052 D84ED0                          FMUL r9         // r9 y; r8 x; x; y; z; w;
00000055 DEC1                            FADDP ST(1),ST  // r8 x + r9 y; x; y; z; w;
00000057 D9C3                            FLD ST(3)       // z; r8 x + r9 y; x; y; z; w;
00000059 D84ECC                          FMUL r10        // r10 z; r8 x + r9 y; x; y; z; w;
0000005C DEC1                            FADDP ST(1),ST  // r8 x + r9 y + r10 z; x; y; z; w;
0000005E D9C4                            FLD ST(4)       // w; r8 x + r9 y + r10 z; x; y; z; w;
00000060 D84EC8                          FMUL r11        // r11 w; r8 x + r9 y + r10 z; x; y; z; w;
00000063 DEC1                            FADDP ST(1),ST  // r8 x + r9 y + r10 z + r10 w; x; y; z; w;
00000065 DD19                            FSTP z          // z = r8 x + r9 y + r10 z + r10 w, x; y; z; w;

00000067 D84EC4                          FMUL r12        // r12 x; y; z; w;
0000006A D9C9                            FXCH ST(1)      // y; r12 x; z; w;
0000006C D84EC0                          FMUL r13        // r13 y; r12 x; z; w;
0000006F DEC1                            FADDP ST(1),ST  // r12 x + r13 y; z; w;
00000071 D9C9                            FXCH ST(1)      // z; r12 x + r13 y; w;
00000073 D84EBC                          FMUL r14        // r14 z; r12 x + r13 y; w;
00000076 DEC1                            FADDP ST(1),ST  // r12 x + r13 y + r14 z; w;
00000078 D9C9                            FXCH ST(1)      // w; r12 x + r13 y + r14 z;
0000007A D84EB8                          FMUL r15        // r15 w; r12 x + r13 y + r14 z;
0000007D DEC1                            FADDP ST(1),ST  // r12 x + r13 y + r14 z + r15 w;
0000007F DD5908                          FSTP w          // w = r12 x + r13 y + r14 z + r15 w

00000082 5E                              POP ESI

00000083 5D                              POP EBP
00000084 C20800                          RET 0008
```

## Code

```rust
fn rotate4d(v: &mut Vec4<f64>,yz: f64,xz: f64,xy: f64,xw: f64,yw: f64,zw: f64) {
    let mut mat = Mat4::<f64>::ONE;
    mat.y.y = cos(yz);
    mat.z.z = cos(yz);
    mat.y.z = -sin(yz);
    mat.z.y = sin(yz);
    let mat2 = Mat4::<f64>::ONE;
    mat2.x.x = cos(xz);
    mat2.z.z = cos(xz);
    mat2.x.z = -sin(xz);
    mat2.z.x = sin(xz);
    mat *= mat2;
    let mat2 = Mat4::<f64>::ONE;
    mat2.x.x = cos(xy);
    mat2.y.y = cos(xy);
    mat2.x.y = -sin(xy);
    mat2.y.x = sin(xy);
    mat *= mat2;
    let mat2 = Mat4::<f64>::ONE;
    mat2.x.x = cos(xw);
    mat2.w.w = cos(xw);
    mat2.x.w = -sin(xw);
    mat2.w.x = sin(xw);
    mat *= mat2;
    let mat2 = Mat4::<f64>::ONE;
    mat2.y.y = cos(yw);
    mat2.w.w = cos(yw);
    mat2.y.w = -sin(yw);
    mat2.w.y = sin(yw);
    mat *= mat2;
    let mat2 = Mat4::<f64>::ONE;
    mat2.z.z = cos(zw);
    mat2.w.w = cos(zw);
    mat2.z.w = -sin(zw);
    mat2.z.w = sin(zw);
    mat *= mat2;
    v = mat * v;
}
```
