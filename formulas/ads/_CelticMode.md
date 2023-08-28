# `_CelticMode`

```
[OPTIONS]
.Version = 2
.DEoption = -1
.Double Good rune = -1
.Double Evil rune = 0
.Double Mystic rune = 0
.Double Heretic rune = 0

[CODE]
558BEC53505657908B75088BD88B7E30DD03DD03DD4618DC4FF0DEC1D9E1DD46
18DC4FF0D9E0DEC1D9C9DC4FE8DEC1DD1BDD47E0D9EEDED9DFE080E44180F440
7418DD01DD4628DC4FE0DEC1D9E1DD4628DC4FD8D9E0DEC1DD19D9D08BC35F5E
585B8BE55DC20800
[END]

Description:

This effect works at best with every formula that at z=0 show a Mandelbrot set, and if it's at power 2.
The M-set must be correctly oriented; I taken "Integer power" as reference.
Most formulas have this orientation. This works perfectly with 4-D formulas as well!
Too bad, KaliLinComb and Riemann have non-standard orientation and they will not work.
Put this effect AFTER that formula. If done correctly, you should see a 3D Celtic set!
You can easily obtain the "Druid set". Follow these instructions

DRUID MODE 1 (normal)
Formula1 = Int power or whatever you like, itercount = 1
Formula2 = This transformation, itercount = 1
Formula3 = Same as formula 1.
DRUID MODE 1 (alternate)
Formula1 = Int power or whatever you like, itercount = 2
Formula2 = This transformation, itercount = 1.

If you increase the power, you get higher order celtic sets, less recognizable but also interesting.
Julia sets are also very promising. Cut also at z=0 to see very interesting things...

Celtic formula was invented by Paul Carlson.

Formula:
x' = abs(x+Cx*G)-Cx*G + x*E
if M>0
z' = abs(z+Cz*M)-Cz*H
endif

Unlike UF Celtic set, this set is "conservative" (preserves orientation of the fractal) - so is also perfect for hybrids
```
