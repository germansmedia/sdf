# DETAILS

## Main Passes

1. Sample distance at one specific direction

One march (or a small amount) to establish the distance from the viewer to the fractal.

Data structures needed:

- view configuration
- formula parameters
- march parameters

Renders to storage buffer.

2. Sample depth and ambient occlusion, progressive

Progressive march to build a depth and AO map in the view. The idea is that this is very fast and gives the viewer a good understanding of the fractal.

- view configuration
- progress management
- formula parameters
- march parameters

Renders to RG32F

3. Sample lighting, shadows, etc.

Progressive march to build a full rendering in the view. These are all the other functions that take up more time, lighting, shadows, texturing, coloring, reflections, etc. The result should be the "final output" (we'll do separate passes in later stages of the project).

- view configuration
- progress management
- formula parameters
- march parameters
- lighting, coloring, etc.

Renders to RGBA16F

## Current State (encoded as push constants in the command buffers)

- eye
- cube face
- anisotropy mode (equirect 180 and 360 only)
- anisotropy Y-offset (equirect 180 and 360 only)
(- depth-of-field parameters)

## View Configuration

Structure that contains all parameters related to the view.

- View Types: Quad, Cube, Cylinder 180, Cylinder 360, Equirectangular 180, Equirectangular 360, Fisheye 180, Cylinder
- Stereo: Yes/No
- Camera parameters (quad only), FOV
- Output extent

## Progress Management

Parameters related to rendering progress management.

- progress step (see constants)
- pixel offset (X,Y)

## Formula Parameters

TBD, currently hardcoded. This should contain things like which formulas to mix, or which precompiled shader code to use, or something like that. I haven't decided yet.

## March Parameters

- pose
- scale
- horizon
- escape
- de_stop
(- de_stop_factor)
- max steps
- max iterations

## Lighting, Coloring, etc.

- albedo color
- key light position
- key light color
- shadow power
(- sky light position)
- ambient light color
- background color
- glow color
- fog settings
