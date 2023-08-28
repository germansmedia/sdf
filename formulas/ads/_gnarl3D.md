# `_gnarl3D`, `_gnarl3Dfast`

```
[OPTIONS]
.Version = 2
.DEoption = -1
.Double Strength = 1.03
.Double X step = 0.1
.Double Y step = 0.1
.Double Z step = 0.1
.Double Alpha = 3.0
.Double Beta = 2.0

[CODE]
558BEC568B75088B7630DD01DD46C8DEC9D9FEDD01DEC1DD46D0DEC9D9FEDD01
DEC1D9FEDD46E8DEC9D9E0DD00DEC1DD46F0DEC9DD00DD46C8DEC9D9FEDD00DE
C1DD46D0DEC9D9FEDD00DEC1D9FEDD46E0DEC9D9E0DD02DEC1DD46F0DEC9DD02
DD46C8DEC9D9FEDD02DEC1DD46D0DEC9D9FEDD02DEC1D9FEDD46D8DEC9D9E0DD
01DEC1DD46F0DEC9DD19DD1ADD185E5DC20800
[END]


Description:

Gnarl deformation in 3D (2D formula is from Mark Townsend) - variation with step X, Y, Z and strength
Gnarl formula gives strange "smoke-like" shapes.
Formula by Luca GN 2011
TIPS
Use tiny steps, alpha and beta near to 1, strength = 1 for a "gentle" transformation.
The more you go far the more you give power to deformation; standard values are a good base to start.
To "learn" how the deform work use it alone at first with a cutting plane then apply it to other formulas.
The formula can be hybridated with all formulas including IFS ones.
Handle with care. This transform uses massively sin - it is slow and may be fuzzy

x' = (x - stepX sin(z + sin( a (z + sin (b z )))) * Strength
y' = (y - stepY sin(x + sin( a (x + sin (b x )))) * Strength
z' = (z - stepZ sin(y + sin( a (y + sin (b y )))) * Strength
```

```
[OPTIONS]
.Version = 2
.DEoption = -1
.Double Strength = 1.03
.Double X step = 0.1
.Double Y step = 0.1
.Double Z step = 0.1
.Double Alpha = 3.0
.Double Beta = 2.0
[CONSTANTS]
Double = 0.31830988618379067153776752674503
Double = 3.1415926535897932384626433832795
Double = -5.167375251323450
Double = 2.543310591226590
Double = -0.557168173477934
[CODE]
558BEC5657538B750889C38B7E3083C4E0DD01DD47C8DEC9E89F000000DD01DE
C1DD47D0DEC9E891000000DD01DEC1E888000000DD47E8DEC9D9E0DD03DEC1DD
47F0DEC9DD03DD47C8DEC9E86C000000DD03DEC1DD47D0DEC9E85E000000DD03
DEC1E855000000DD47E0DEC9D9E0DD02DEC1DD47F0DEC9DD02DD47C8DEC9E839
000000DD02DEC1DD47D0DEC9E82B000000DD02DEC1E822000000DD47D8DEC9D9
E0DD01DEC1DD47F0DEC9DD19DD1ADD1B83C42089D85B5F5E5DC20800DC0FDB54
2418DB442418DEE9D9C0D8C8DD4720D8C9DC4718D8C9DC4710DEC9DC4708DEC9
8A44241824017402D9E0C3
[END]


Description:

See Gnarl3D, this is a polynomial version,
faster and accurate (1e-6 MAX error, stopping at x^7) 
- no visible difference in practice!
No sin() functions are used.
The numerical coefficients are taken from
the tables found here:

http://www.geometrictools.com/GTEngine/Include/GteConstants.h
```
