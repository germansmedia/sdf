# `_LinCombineXYZ`

```
[OPTIONS]
.Version = 2
.DEoption = -1
.Double Xx mul = 1
.Double Xy mul = 0
.Double Xz mul = 0
.Double Yx mul = 0
.Double Yy mul = 1
.Double Yz mul = 0
.Double Zx mul = 0
.Double Zy mul = 0
.Double Zz mul = 1
[CODE]
558BEC568B75088B7630DD01DD02DD00D9C0DC4EF0D9C2DC4EE8DEC1D9C3DC4E
E0DEC1DD18D9C0DC4ED8D9C2DC4ED0DEC1D9C3DC4EC8DEC1DD1ADC4EC0D9C9DC
4EB8DEC1D9C9DC4EB0DEC1DD195E5DC20800
[END]

Description:

A general 3x3 matrix conversion:

x' = Xx*x + Xy*y + Xz*z
y' = Yx*x + Yy*y + Yz*z
z' = Zx*x + Zy*y + Zz*z
```

## Code

```glsl
v = mat * v;
```
