# `_Translate`

```
[OPTIONS]
.Version = 2
.DEoption = -1
.Double X add = 0
.Double Y add = 0
.Double Z add = 0
[CODE]
558BEC568B75088B7630DD01DD02DD00DC46F0DD18DC46E8DD1ADC46E0DD195E
5DC20800
[END]
```

## Disassembly

[eax] = v.x
[edx] = v.y
[ecx] = v.z
[esi-0x10] = add.x
[esi-0x18] = add.y
[esi-0x20] = add.z

```
0:  55                      push   ebp
1:  8b ec                   mov    ebp,esp

3:  56                      push   esi

4:  8b 75 08                mov    esi,DWORD PTR [ebp+0x8]
7:  8b 76 30                mov    esi,DWORD PTR [esi+0x30]

a:  dd 01                   fld    v.z
c:  dd 02                   fld    v.y
e:  dd 00                   fld    v.x
10: dc 46 f0                fadd   add.x
13: dd 18                   fstp   v.x
15: dc 46 e8                fadd   add.y
18: dd 1a                   fstp   v.y
1a: dc 46 e0                fadd   add.z
1d: dd 19                   fstp   v.z

1f: 5e                      pop    esi

20: 5d                      pop    ebp
21: c2 08 00                ret    0x8
```

## Code

```glsl
v += add;
```
