# `_reciprocalZ3b`

```
[OPTIONS]
.Version = 4
.DEoption = -1
.DReci2 Limiter1 = 0.67
.DReci2 Limiter2 = 0.67
.Double mul1 = 1
.Double mul2 = 1
[CODE]
558BEC5652DD018B75088A51078B7630DC4ED0D9E1DC46F0D9E8DEF1D9E0DC46
E8DD01D8C8DC4EC8D9E1DC46E080E280D9E8DEF1D9E0DC46D8DEC1DD19305107
5A5E5DC20800
[END]


Description:

Computes a reciprocal-style function of z with a different (continuous) function,
fine-tuneable:

z' = sign(z)(1/Lim1 + 1/lim2 - 1/(abs(z)+ Lim ) - 1/(z*z+ Lim2 ) )


The DE is almost untouched, it should work fine also with a Raystep 0.5 !
Just don't go too low with the limiter to avoid noise.
```

## Disassembly

`[EBX]` = x
`[EDX]` = y
`[ECX]` = z
`[ESI-08]` = 0.5
`[ESI-10]` = Limiter1
`[ESI-18]` = 1 / Limiter1
`[ESI-20]` = Limiter2
`[ESI-28]` = 1 / Limiter2
`[ESI-30]` = mul1
`[ESI-38]` = mul2

```
00000000 55                              PUSH EBP
00000001 8BEC                            MOV EBP,ESP

00000003 56                              PUSH ESI
00000004 52                              PUSH EDX

00000005 DD01                            FLD z                       // z;
00000007 8B7508                          MOV ESI,DWORD PTR [EBP+08]
0000000A 8A5107                          MOV DL,BYTE PTR [ECX+07]    // get msb from z
0000000D 8B7630                          MOV ESI,DWORD PTR [ESI+30]
00000010 DC4ED0                          FMUL mul1                   // z*mul1;
00000013 D9E1                            FABS                        // |z*mul1|;
00000015 DC46F0                          FADD Limiter1               // |z*mul1|+Limiter1;
00000018 D9E8                            FLD1                        // 1; |z*mul1|+Limiter1;
0000001A DEF1                            FDIVRP ST(1),ST             // 1/(|z*mul1|+Limiter1);
0000001C D9E0                            FCHS                        // -1/(|z*mul1|+Limiter1);
0000001E DC46E8                          FADD 1/Limiter1             // -1/(|z*mul1|+Limiter1)+1/Limiter1;
00000021 DD01                            FLD z                       // z; -1/(|z*mul1|+Limiter1)+1/Limiter1;
00000023 D8C8                            FMUL ST,ST(0)               // zz; -1/(|z*mul1|+Limiter1)+1/Limiter1;
00000025 DC4EC8                          FMUL mul2                   // zz*mul2; -1/(|z*mul1|+Limiter1)+1/Limiter1;
00000028 D9E1                            FABS                        // |zz*mul2|; -1/(|z*mul1|+Limiter1)+1/Limiter1;
0000002A DC46E0                          FADD Limiter2               // |zz*mul2|+Limiter2; -1/(|z*mul1|+Limiter1)+1/Limiter1;
0000002D 80E280                          AND DL,80                   // isolate sign bit from z
00000030 D9E8                            FLD1                        // 1; |zz*mul2|+Limiter2; -1/(|z*mul1|+Limiter1)+1/Limiter1;
00000032 DEF1                            FDIVRP ST(1),ST             // 1/(|zz*mul2|+Limiter2); -1/(|z*mul1|+Limiter1)+1/Limiter1;
00000034 D9E0                            FCHS                        // -1/(|zz*mul2|+Limiter2); -1/(|z*mul1|+Limiter1)+1/Limiter1;
00000036 DC46D8                          FADD 1/Limiter2             // -1/(|zz*mul2|+Limiter2)+1/Limiter2; -1/(|z*mul1|+Limiter1)+1/Limiter1;
00000039 DEC1                            FADDP ST(1),ST              // -1/(|z*mul1|+Limiter1)+1/Limiter1-1/(|zz*mul2|+Limiter2)+1/Limiter2;
0000003B DD19                            FSTP QWORD PTR [ECX]        // z = -1/(|z*mul1|+Limiter1)+1/Limiter1-1/(|zz*mul2|+Limiter2)+1/Limiter2
0000003D 305107                          XOR BYTE PTR [ECX+07],DL    // flip sign of new z according to sign bit from original z, i.e. multiply by sign(z)

00000040 5A                              POP EDX
00000041 5E                              POP ESI

00000042 5D                              POP EBP
00000043 C20800                          RET 0008
```

## Code

```rust
let c = 1.0 / limiter1 + 1.0 / limiter2;
let l = 1.0 / ((mul1 * z).abs() + limiter1);
let q = 1.0 / ((mul2 * z * z).abs() + limiter2);
z = sign(z) * (c - l - q);
```
