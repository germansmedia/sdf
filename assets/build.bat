@echo off
cd glsl
glslc -O -fshader-stage=compute yardstick_cs.glsl -o ..\yardstick.spirv
glslc -O -fshader-stage=compute viewer_depth_occlusion_cs.glsl -o ..\viewer_depth_occlusion.spirv
glslc -O -fshader-stage=compute viewer_lighting_cs.glsl -o ..\viewer_lighting.spirv
glslc -O -fshader-stage=compute tiler_cs.glsl -o ..\tiler.spirv
glslc -O -fshader-stage=vertex projector_equirect_vs.glsl -o ..\projector_equirect_vs.spirv
glslc -O -fshader-stage=fragment projector_equirect_fs.glsl -o ..\projector_equirect_fs.spirv
cd ..
