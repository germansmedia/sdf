# M3D TEXT FORMAT

## General Formatting

The data is stored between `Mandelbulb3Dv18{\n` and `}\n`, followed by one line specifying the title `{Titel: $title}\n`.

## Base64-ish Encoding

The binary data is encoded in a similar way to base64. This means every 3 bytes are written as 4 characters, using the following translation table:

|| 00.... | 01.... | 10.... | 11....|
|:-:|:-:|:-:|:-:|:-:|
|..0000 | `.` | `E` | `U` | `k` |
|..0001 | `/` | `F` | `V` | `l` |
|..0010 | `0` | `G` | `W` | `m` |
|..0011 | `1` | `H` | `X` | `n` |
|..0100 | `2` | `I` | `Y` | `o` |
|..0101 | `3` | `J` | `Z` | `p` |
|..0110 | `4` | `K` | `a` | `q` |
|..0111 | `5` | `L` | `b` | `r` |
|..1000 | `6` | `M` | `c` | `s` |
|..1001 | `7` | `N` | `d` | `t` |
|..1010 | `8` | `O` | `e` | `u` |
|..1011 | `9` | `P` | `f` | `v` |
|..1100 | `A` | `Q` | `g` | `w` |
|..1101 | `B` | `R` | `h` | `x` |
|..1110 | `C` | `S` | `i` | `y` |
|..1111 | `D` | `T` | `j` | `z` |

So one character represents 6 bits. 4 characters concatenate to 24 bits, which are 3 bytes in the binary data. Note that both the 4 characters and the resulting 3 bytes are little-endian. If the bits in the 4 characters are *a5a4a3a2a1a0*, *b5b4b3b2b1b0*, *c5c4c3c2c1c0* and *d5d4d3d2d1d0*, then the resulting bytes are *d5d4d3d2d1d0c5c4*, *c3c2c1c0b5b4b3b2* and *b1b0a5a4a3a2a1a0*.

## Binary Data

### `TMandHeader10` and `TLightingParas8`

| offset | name | type | details |
|-:|:-|:-|:-|
| 0000 | MandId     | u32  | |
| 0004 | Width      | u32  | |
| 0008 | Height     | u32  | |
| 000C | Iterations | u32  | |
| 0010 | iOptions   | u16  | bit1: FirstStepRandom, bit3: StepSubDEstop,? |
| 0012 | bNewOptions | u8 | bit1: use quaternion iso. rotation matrix, bit2: color on it nr |
| 0013 | bColorOnIt | u8 | 00: disabled, ??: outputvec = inputvec (1)2..255 iterate n-1 times + docolor |
| 0014 | dZstart | f64 | |
| 001C | dZend | f64 | |
| 0024 | dXmid | f64 | |
| 002C | dYmid | f64 | |
| 0034 | dZmid | f64 | |
| 003C | dXWrot | f64 | 4D rotation |
| 0044 | dYWrot | f64 | |
| 004C | dZWrot | f64 | |
| 0054 | dZoom | f64 | |
| 005C | RStop | f64 | |
| 0064 | iReflectsCalcTime | i32 | in MCmode: OldAvrgRayCount |
| 0068 | sFmixPow | f32 | for formula DE Mix combs |
| 006C | dFOVy | f64 | in single? |
| 0074 | sTRIndex | f32 | for transmission calculation |
| 0078 | sTRscattering | f32 | light scattering amount |
| 007C | MCoptions | u8 | bit1: HDR, bit2: bSecantSearch, bit3: autoclipS+D, bit5..7: DoFbokeh, bit8: newMCrecordYUV |
| 007D | MCdiffReflects | u8 | D2Byte 0.00 .. 2.50, reflects diffusity, bit4: aa box/gauss |
| 007E | bStereoMode | u8 | 0: no, 1: very left, 3: right, 4: left |
| 007F | bSSAO24BorderMirrorSize | u8 | 0 to 0.5 |
| 0080 | iAmbCalcTime | u32 | |
| 0084 | bNormalsOnDE | u8 | |
| 0085 | bCalculateHardShadow | u8 | bit1: calc automatic, bit2: setLdifFuncToCos, bit3..bit8: light1-6 |
| 0086 | bStepsafterDEStop | u8 | bin search |
| 0087 | MinimumIterations | u16 | |
| 0089 | MClastY | i16 | |
| 008B | bCalc1HSsoft | u8 | option to calculate only 1HS but 6 bit as amount |
| 008C | iAvrgDEsteps | u32 | val * 10 |
| 0090 | iAvrgIts | u32 | |
| 0094 | bPlanarOptic | u8 | 0,1: camera planar optic, 2: spherePano, 3: dome? |
| 0095 | bCalcAmbShadowAutomatic | u8 | bit1: yes/no, bit2: kindof threshold maxclib/thr. down to 0, bit 3+4: type (0: 15bit, 1: 24bit, 2: 24bit rand, 3: DEAO) |
| 0096 | sNaviMinDist | f32 | bit 5+6+7: DE raycount (3, 7, 17, 33), bit8: FSR (first step random) |
| 009A | dStepWidth | f64 | related to zoom |
| 00A2 | bVaryDEstopOnFOV | u8 | |
| 00A3 | bHScalculated | u8 | if it was calculated, 6 bits yes/no of light 1-6 = bit3..8 |
| 00A4 | sDOFzsharp | f32 | |
| 00A8 | sDOFclipR | f32 | |
| 00AC | sDOFaperture | f32 | |
| 00B0 | bCutOption | u8 | |
| 00B1 | sDEstop | f32 | |
| 00B5 | bCalcDOFtype | u8 | 0: don't calc, bit 2+3: passed, bit4: function sorted/forward |
| 00B6 | mZstepDiv | f32 | |
| 00BA | MCDepth | i8 | |
| 00BB | SSAORcount | u8 | |
| 00BC | AODEdithering | u8 | |
| 00BD | bImageScale | u8 | |
| 00BE | bIsJulia | u8 | |
| 00BF | dJx | f64 | Julia vals |
| 00C7 | dJy | f64 | |
| 00CF | dJz | f64 | |
| 00D7 | dJw | f64 | |
| 00DF | bDFogIt | u8 | |
| 00E0 | MCSoftShadoRadius | f16 | |
| 00E2 | HSmaxLengthMultiplier | f32 | |
| 00E6 | StereoScreenWidth | f32 | |
| 00EA | StereoScreenDistance | f32 | |
| 00EE | StereoMinDistance | f32 | |
| 00F2 | sRaystepLimiter | f32 | |
| 00F6 | hVGrads | Mat3x3<f64> | complete 3x3 nav matrix, can change to quaternion if wNewOptions and 1 |
| 013E | bMCSaturation | u8 | |
| 013F | sAmbShadowThreshold | f32 | z/r |
| 0143 | iCalcTime | u32 | Seconds * 10 |
| 0147 | iCalcHStime | u32 | |
| 014B | byCalcNsOnZBufAuto | u8 | |
| 014C | SRamount | f32 | Amount of reflection light |
| 0150 | bCalcSRautomatic | u8 | bit1: auto, bit2: trans, bit3: only dIFS |
| 0151 | SRreflectioncount | u8 | |
| 0152 | sColorMul | f32 | multiplier for color option 'last vector increase' |
| 0156 | byColor2Option | u8 | |
| 0157 | bVolLightNr | u8 | was: byRepeatFrom -> in HAddon lower 3(4) bits: lightnr, upper 4 bits: mapsize +/-7 in 20% steps |
| 0158 | bCalc3D | u8 | |
| 0159 | bSliceCalc | u8 | |
| 015A | dCutX | f64 | |
| 0162 | dCutY | f64 | |
| 016A | dCutZ | f64 | |
| 0172 | sTransmissionAbsorption | f32 | |
| 0176 | sDEAOmaxL | f32 | |
| 017A | sDEcombS | f32 | DEcombAvrg absolute smooth distance; was pointer to custom formula.. obsolete |
| 017E | PHcustomF | [u32; 6] | just 1 pointer to formulastruct |
| 0196 | PCFAddon | u32 | pointer to the Header Addon for the customF's data |
| 019A | sDOFZsharp2 | f32 | 2nd focuspoint, but store user name here.. |
| 019E | iMaxIts | u32 | |
| 01A2 | iMaxItsF2 | u32 | DEcomb maxits for formula 2 |
| 01A6 | DEmixColorOption | u8 | |
| 01A7 | MCcontrast | u8 | |
| 01A8 | sM3dVersion | f32 | |
| 01AC | TilingOptions | u32 | in MCmode: OldAvrgSqrNoise as Single |
| 01B0 | VarColZpos | i16 | is trackbar -> smallint |
| 01B2 | RoughnessFactor | u8 | 0..1 |
| 01B3 | bColorMap | u8 | second byte in DiffMapNrEx |
| 01B4 | DynFogCol2 | Vec3<u8> | |
| 01B7 | AdditionalOptions | u8 | bit1: Internal Gamma of 2;  bit8: convertBGpicTospherical (bit 5..7: fit border width on load 0..7) ->only 1 bit +1bits for ambient option small bgpic as ambient! +1 |
| 01B8 | TBpos | [i32; 9] | |
| 01DC | TBoptions | u32 | 1-7.bit TB12pos; 8-14.bit TB13pos = interior col pos;  bit 15 = color cycling;  bit16: BGimageDirectCoord;  bit17 = fineAdjDown
| 01E0 | FineColAdj1 | u8 | |
| 01E1 | FineColAdj2 | u8 | |
| 01E2 | PicOffsetX | u8 | |
| 01E3 | PicOffsetY | u8 | |
| 01E4 | AmbCol | Vec3<u8> | |
| 01E7 | DynFogR | u8 | |
| 01E8 | AmbCol2 | Vec3<u8> | |
| 01EB | DynFogG | u8 | |
| 01EC | DepthCol | Vec3<u8> | |
| 01EF | DynFogB | u8 | |
| 01F0 | DepthCol2 | Vec3<u8> | |
| 01F3 | PicOffsetZ | u8 | |
| 01F4 | Lights | [TLight8; 6] | |
| 02B4 | LCols | [TLCol8; 10] | |
| 0318 | ICols | [TICol8; 4] | |
| 0330 | BGbmp | [u8; 24] | background image file name |

### `TLight8`

| offset | name | type | details |
|-:|:-|:-|:-|
| 0000 | Loption | u8 | bit1: 0: On  1: Off;  bit2: lightmap;  bit3 = bPosLight, bit4+5 = poslight visible+func, bit6 = global light rel to object, bit7 = HSon |
| 0001 | Lfunction | u8 | 4bit spec func + 2bit diff,  Spec expo = 8 shl (LFunction and $07), diff = (LFunction shr 4) and 3     ..+ 1 bit for extVisLight(s)? |
| 0002 | Lamp | u16 | Light amplitude for posLight -> exp 8bit shortint + 8bit byte mant for wide range! -> for all lights |
| 0004 | Lcolor | Vec3<u8> | |
| 0007 | LightMapNr | u16 | 0: no LM, 1..32000: LMnr,  LM works as ambient light was byte, now with ..Ex as word! |
| 0009 | LXpos | [u8; 7] | |
| 0010 | AdditionalByteEx | u8 | LVersionEx in Light[0] and DiffMapNrEx in Light[1], diffmap scale in Light[2], diff shadowing in Light[3], BGscale in Light[4] |
| 0011 | LYpos | [u8; 7] | |
| 0018 | FreeByte | u8 | iColOnOT := 2 + (HLight.Lights[1].FreeByte and 3);  HLight.Lights[0].FreeByte for bgpic options |
| 0019 | LZpos | [u8; 7] | |
| 001F | FreeByte | ? | unclear if part of TLight8 |

### `TLCol8`

| offset | name | type | details |
|-:|:-|:-|:-|
| 0000 | Position | u16 | |
| 0002 | ColorDif | u32 | |
| 0004 | ColorSpe | u32 | |

### `TICol8`

| offset | name | type | details |
|-:|:-|:-|:-|
| 0000 | Position | u16 | |
| 0002 | Color | u32 | |

### `THeaderCustomAddon`

When specific formulas are used, the data is followed by this header.

| offset | name | type | details |
|-:|:-|:-|:-|
| 0348 | bHCAversion | u8 | |
| 0349 | bOptions1 | u8 | type of hybrid: 0 = alt, 1 = interpolhybrid, 2 = decombinated, 3 = K/L?IFS |
| 034A | bOptions2 | u8 | bit1: disable analytical DE, bit2+3: 0: outside, 1: inside, 2: in+outside rendering; bit4(+5): map interpolation cos/bicubic
| 034B | bOptions3 | u8 | bit1+2+3: type of DEcombination |
| 034C | iFCount   | u8 | max formula index |
| 034D | bHybOpt1  | u8 | nibbles: end1, repeat1 |
| 034E | bHybOpt2  | u16 | nibbles: start2, end2, repeat2 |
| 0350 | Formulas  | [THAformula; ifCount] | the formula-specific data |

### `THAformula`

| offset | name | type | details |
|-:|:-|:-|:-|
| 0000 | iItCount | i32 | |
| 0004 | iFnr | i32 | intern < 20 nr of formula, extern 20 or higher: CustomFname for identification |
| 0008 | iOptionCount | i32 | |
| 000C | CustomFname | [u8; 32] | name of the formula, if external |
| 002C | byOptionType | [u8; 16] | type of each option |
| 003C | dOptionValue | [f64; 16] | option values |

