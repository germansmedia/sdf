# `_FlipXYc`

```
[OPTIONS]
.Version = 2
.DEoption = -1
.Integer Flip also C = 0
[CODE]
558BEC56578B75088B7E30837FF400740CDD4618DD4620DD5E18DD5E20DD02DD
00DD1ADD185F5E5DC20800
[END]


Description:

Exchanging the x and y values, if option is not zero then Cx and Cy too:

x' = y
y' = x

if Flip_also_C <> 0 {Cx' = Cy; Cy' = Cx}

Note:
Changing C does not work in Julia mode.
```

## Disassembly

[edi-0xc] = flip
[esi+0x18] = c.x
[esi+0x20] = c.y
[edx] = v.y
[eax] = v.x

```
0:  55                      push   ebp
1:  8b ec                   mov    ebp,esp

3:  56                      push   esi
4:  57                      push   edi

5:  8b 75 08                mov    esi,DWORD PTR [ebp+0x8]
8:  8b 7e 30                mov    edi,DWORD PTR [esi+0x30]

b:  83 7f f4 00             cmp    DWORD PTR [edi-0xc],0x0
f:  74 0c                   je     0x1d

11: dd 46 18                fld    QWORD PTR [esi+0x18]
14: dd 46 20                fld    QWORD PTR [esi+0x20]
17: dd 5e 18                fstp   QWORD PTR [esi+0x18]
1a: dd 5e 20                fstp   QWORD PTR [esi+0x20]

1d: dd 02                   fld    QWORD PTR [edx]
1f: dd 00                   fld    QWORD PTR [eax]
21: dd 1a                   fstp   QWORD PTR [edx]
23: dd 18                   fstp   QWORD PTR [eax]

25: 5f                      pop    edi
26: 5e                      pop    esi

27: 5d                      pop    ebp
28: c2 08 00                ret    0x8
```

## Code

```glsl
if (flip) {
    FLOAT temp = c.x;
    c.x = c.y;
    c.y = temp;
}
FLOAT temp = v.x;
v.x = v.y;
v.y = temp;
```
