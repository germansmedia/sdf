# `_SinY`

```
[OPTIONS]
.Version = 2
.DEoption = -1
.Integer Index = 1
.Single Offset 1 = 0
.Single Scale 1 = 1
.Single Scale 2 = 1
.Single Offset 2 = 0
[CODE]
558BEC56578B75088B7E308B77F483E603DD04F0D867F0D84FECD9FED84FE8D8
47E4DD1CF05F5E5DC20800
[END]

New added parameters:

Index:
Do this function on vectorpart: 

0: x
1: y
2: z
3: w


The function itself:

a = Sin((a - offset1) * scale1) * scale2 + offset2
```

## Disassembly

[edi-0xc] = index
[edi-0x10] = offset1
[edi-0x14] = scale1
[edi-0x18] = scale2
[edi-0x1c] = offset2

```
0:  55                      push   ebp
1:  8b ec                   mov    ebp,esp

3:  56                      push   esi
4:  57                      push   edi

5:  8b 75 08                mov    esi,DWORD PTR [ebp+0x8]
8:  8b 7e 30                mov    edi,DWORD PTR [esi+0x30]

b:  8b 77 f4                mov    esi,DWORD PTR [edi-0xc]
e:  83 e6 03                and    esi,0x3
11: dd 04 f0                fld    QWORD PTR [eax+esi*8]
14: d8 67 f0                fsub   offset1
17: d8 4f ec                fmul   scale1
1a: d9 fe                   fsin
1c: d8 4f e8                fmul   scale2
1f: d8 47 e4                fadd   offset2
22: dd 1c f0                fstp   QWORD PTR [eax+esi*8]

25: 5f                      pop    edi
26: 5e                      pop    esi

27: 5d                      pop    ebp
28: c2 08 00                ret    0x8
```

## Code

```glsl
switch (index) {
    case 0: v.x = sin((v.x - offset1) * scale1) * scale2 + offset2; break;
    case 1: v.y = sin((v.y - offset1) * scale1) * scale2 + offset2; break;
    case 2: v.z = sin((v.z - offset1) * scale1) * scale2 + offset2; break;
    case 3: v.w = sin((v.w - offset1) * scale1) * scale2 + offset2; break;
}
```
