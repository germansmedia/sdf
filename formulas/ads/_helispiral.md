# `_helispiral`

```
[OPTIONS]
.Version = 2
.DEoption = -1
.3SingleAngles Reference rotation = 0
.Double Helix = .03
.Double Spiral = .03
.Integer RotByC (H) = 0
.Integer RotByC (S) = 0
.Double FixRot (deg) = 0
[CONSTANTS]
Double = 0.017453293
[CODE]
558BEC5657538B750889C38B7E30DD01
DD02DD03D9C0D84FDCD9C2D84FD8DEC1
D9C3D84FD4DEC1D9C1D84FF4D9C3D84F
F0DEC1D9C4D84FECDEC1D9CAD84FE8D9
CBD84FE4DEC3D9CBD84FE0DEC2DD1BDD
1ADD19D9D0DD02DD03837FBC01750ADD
4618D8C8DD4620EB06D9C0D8C8DD02D8
C8DEC1D9FADD47C4DEC9837FC0017505
DD4628EB02DD01DD47CCDEC9DEC1DD47
B4DC0F90DEC1D9FBD9C0D8CBD9C2D8CD
DEC1DD1BDECBDEC9DEE9DD1AD9D0DD01
DD02DD03D9C0D84FECD9C2D84FE0DEC1
D9C3D84FD4DEC1D9C1D84FF4D9C3D84F
E8DEC1D9C4D84FDCDEC1D9CAD84FF0D9
CBD84FE4DEC3D9CBD84FD8DEC2DD1BDD
1ADD1989D85B5F5E5DC20800
[END]


Description:
1)Rotates the reference axis by selected angles
NOTE; this will NOT affect C!
2)Rotates the XY plane on an angle defined as follows:
if rbcH=1 { phi = Cz } else { phi = z }
if rbcS=1 { rho = sqrt(Cx*Cx+Cy*Cy) } else { rho = sqrt(x*x+y*y) }

SinCos(FixRot*degtorad + rho*Spiral + phi*Helix, s, c)
x' = x * c + y * s
y' = y * c - x * s

3)Rotates the reference axis at the inital position (transposed rot. matrix)
NOTE; this will NOT affect C!

This gives a helix + spiral deformation (if used as pretransform,
or else it gives strange results, you choose).

New version, june 21 2011 - Luca GN
```
