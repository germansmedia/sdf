# `_PolyFold-sym`

```
[OPTIONS]
.Version = 2
.DEoption = -1
.Double Order = 4
.Double Shift (deg) = 0
.Double Shift x = 0
.Double Shift y = 0
.Double Skew (deg) = 0
[CONSTANTS]
Double = 6.283185307
Double = 0.017453293
[CODE]
558BEC5657538B750889C38B7E30DD02DC47D8DD12DD03DC47E0DD13D9F3DD47
D0DC47E8DC4F08DEC1DC37DC4FF083C4F090DB1C24DB0424D9E0DC77F0DC0FDD
47E8DC4F08DEC1D9FBD9D0DD03D8C9DD02D8CBDEE1D9D0DD03D8CBDD02D8CBDE
C18B042483E0017502D9E083C410DC67D8DD1ADC67E0DD1BDDD8DDD85B5F5E5D
C20800
[END]

Description:

Useful to get symmetric poly and star effects. Sym because on even orders it is a continue symmetrical transform.
Also you can use Shift X, Y, and the angular shift without causing discontinuity.

Luca GN 2012
```

## Disassembly

`[EBX]` = x
`[EDX]` = y
`[ECX]` = z
`[ESI+18]` = Cx
`[ESI+20]` = Cy
`[ESI+28]` = Cz
`[EDI]` = C0 = 6.283185307 (2pi)
`[EDI+08]` = C1 = 0.017453293 (pi/180)
`[EDI-08]` = 0.5
`[EDI-10]` = Order
`[EDI-18]` = Shift (deg)
`[EDI-20]` = Shift x
`[EDI-28]` = Shift y
`[EDI-30]` = Skew (deg)

```
00000000 55                              PUSH EBP
00000001 8BEC                            MOV EBP,ESP

00000003 56                              PUSH ESI
00000004 57                              PUSH EDI
00000005 53                              PUSH EBX

00000006 8B7508                          MOV ESI,DWORD PTR [EBP+08]
00000009 89C3                            MOV EBX,EAX
0000000B 8B7E30                          MOV EDI,DWORD PTR [ESI+30]

0000000E DD02                            FLD y                    // y;
00000010 DC47D8                          FADD Shift y             // y+sy;
00000013 DD12                            FST y                    // y = y+sy, y+sy;
00000015 DD03                            FLD x                    // x; y+sy;
00000017 DC47E0                          FADD Shift x             // x+sx; y+sy;
0000001A DD13                            FST x                    // x = x+sx, x+sx; y+sy;
0000001C D9F3                            FPATAN                   // atan(y+sy,x+sx);
0000001E DD47D0                          FLD Skew (deg)           // skew; atan(y+sy,x+sx);
00000021 DC47E8                          FADD Shift (deg)         // skew+shift; atan(y+sy,x+sx);
00000024 DC4F08                          FMUL C0                  // (skey+shift)*C0; atan(y+sy,x+sx);
00000027 DEC1                            FADDP ST(1),ST           // atan(y+sy,x+sx)+(skey+shift)*C0;
00000029 DC37                            FDIV C1                  // (atan(y+sy,x+sx)+(skey+shift)*C0)/C1;
0000002B DC4FF0                          FMUL Order               // order*(atan(y+sy,x+sx)+(skey+shift)*C0)/C1;
0000002E 83C4F0                          ADD ESP,FFFFFFF0         // reserve 8 bytes on stack (SUB ESP,8?)
00000031 90                              NOP
00000032 DB1C24                          FISTP DWORD PTR [ESP]    // let t = order*(atan(y+sy,x+sx)+(skey+shift)*C0)/C1 as i32;
00000035 DB0424                          FILD DWORD PTR [ESP]     // t;
00000038 D9E0                            FCHS                     // -t;
0000003A DC77F0                          FDIV Order               // -t/order;
0000003D DC0F                            FMUL C1                  // -C1*t/order;
0000003F DD47E8                          FLD Shift (deg)          // shift; -C1*t/order;
00000042 DC4F08                          FMUL C0                  // C0*shift; -C1*t/order;
00000045 DEC1                            FADDP ST(1),ST           // C0*shift-C1*t/order;
00000047 D9FB                            FSINCOS                  // cos(C0*shift-C1*t/order); sin(C0*shift-C1*t/order);
00000049 D9D0                            FNOP
0000004B DD03                            FLD x                    // x+sx; cos(C0*shift-C1*t/order); sin(C0*shift-C1*t/order);
0000004D D8C9                            FMUL ST,ST(1)            // (x+sx)*cos(C0*shift-C1*t/order); cos(C0*shift-C1*t/order); sin(C0*shift-C1*t/order);
0000004F DD02                            FLD y                    // y+sy; x*cos(C0*shift-C1*t/order); cos(C0*shift-C1*t/order); sin(C0*shift-C1*t/order);
00000051 D8CB                            FMUL ST,ST(3)            // (y+sy)*sin(C0*shift-C1*t/order); x*cos(C0*shift-C1*t/order); cos(C0*shift-C1*t/order); sin(C0*shift-C1*t/order);
00000053 DEE1                            FSUBRP ST(1),ST          // (y+sy)*sin(C0*shift-C1*t/order)-(x+sx)*cos(C0*shift-C1*t/order); cos(C0*shift-C1*t/order); sin(C0*shift-C1*t/order);
00000055 D9D0                            FNOP
00000057 DD03                            FLD x                    // x+sx; (y+sy)*sin(C0*shift-C1*t/order)-(x+sx)*cos(C0*shift-C1*t/order); cos(C0*shift-C1*t/order); sin(C0*shift-C1*t/order);
00000059 D8CB                            FMUL ST,ST(3)            // (x+sx)*sin(C0*shift-C1*t/order); (y+sy)*sin(C0*shift-C1*t/order)-(x+sx)*cos(C0*shift-C1*t/order); cos(C0*shift-C1*t/order); sin(C0*shift-C1*t/order);
0000005B DD02                            FLD y                    // y+sy; (x+sx)*sin(C0*shift-C1*t/order); (y+sy)*sin(C0*shift-C1*t/order)-(x+sx)*cos(C0*shift-C1*t/order); cos(C0*shift-C1*t/order); sin(C0*shift-C1*t/order);
0000005D D8CB                            FMUL ST,ST(3)            // (y+sy)*cos(C0*shift-C1*t/order); (x+sx)*sin(C0*shift-C1*t/order); (y+sy)*sin(C0*shift-C1*t/order)-(x+sx)*cos(C0*shift-C1*t/order); cos(C0*shift-C1*t/order); sin(C0*shift-C1*t/order);
0000005F DEC1                            FADDP ST(1),ST           // (y+sy)*cos(C0*shift-C1*t/order)+(x+sx)*sin(C0*shift-C1*t/order); (y+sy)*sin(C0*shift-C1*t/order)-(x+sx)*cos(C0*shift-C1*t/order); cos(C0*shift-C1*t/order); sin(C0*shift-C1*t/order);
00000061 8B0424                          MOV EAX,DWORD PTR [ESP]  // EAX = t
00000064 83E001                          AND EAX,00000001         // EAX = t & 1
00000067 7502                            JNE 0000006B             // (t&1) != 0

00000069 D9E0                            FCHS                     // change sign if t is even

0000006B 83C410                          ADD ESP,00000010         // restore stack
0000006E DC67D8                          FSUB Shift y             // (y+sy)*cos(C0*shift-C1*t/order)+(x+sx)*sin(C0*shift-C1*t/order)-sy; (y+sy)*sin(C0*shift-C1*t/order)-(x+sx)*cos(C0*shift-C1*t/order); cos(C0*shift-C1*t/order); sin(C0*shift-C1*t/order);
00000071 DD1A                            FSTP y                   // y = (y+sy)*cos(C0*shift-C1*t/order)+(x+sx)*sin(C0*shift-C1*t/order)-sy, (y+sy)*sin(C0*shift-C1*t/order)-(x+sx)*cos(C0*shift-C1*t/order); cos(C0*shift-C1*t/order); sin(C0*shift-C1*t/order);
00000073 DC67E0                          FSUB Shift x             // (y+sy)*sin(C0*shift-C1*t/order)-(x+sx)*cos(C0*shift-C1*t/order)-sx; cos(C0*shift-C1*t/order); sin(C0*shift-C1*t/order);
00000076 DD1B                            FSTP x                   // x = (y+sy)*sin(C0*shift-C1*t/order)-(x+sx)*cos(C0*shift-C1*t/order)-sx, cos(C0*shift-C1*t/order); sin(C0*shift-C1*t/order);
00000078 DDD8                            FSTP ST(0)               // sin(C0*shift-C1*t/order);
0000007A DDD8                            FSTP ST(0)

0000007C 5B                              POP EBX
0000007D 5F                              POP EDI
0000007E 5E                              POP ESI

0000007F 5D                              POP EBP
00000080 C20800                          RET 0008
```

## Code

```rust
let ty = y + shift_y;
let tx = x + shift_x;
let t = order * rad2deg(atan(ty,tx) + 2.0 * M_PI * (skew + shift)) as i32;
let a = 2.0 * M_PI * shift - deg2rad(t as f64) / order;
let sa = sin(a);
let ca = cos(a);
y = ty * ca + tx * sa - shift_y;
if (t & 1) != 0 { y = -y; };
x = ty * sa - tx * ca - shift_x;
```
