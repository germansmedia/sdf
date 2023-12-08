@echo off
cd glsl
glslc -O -fshader-stage=compute measure_cs.glsl -o ..\measure_cs.spirv
glslc -O -fshader-stage=compute depth_occlusion_cs.glsl -o ..\depth_occlusion_cs.spirv
glslc -O -fshader-stage=compute lighting_cs.glsl -o ..\lighting_cs.spirv
glslc -O -fshader-stage=vertex draw_equirect_vs.glsl -o ..\draw_equirect_vs.spirv
glslc -O -fshader-stage=fragment draw_equirect_fs.glsl -o ..\draw_equirect_fs.spirv
cd ..
