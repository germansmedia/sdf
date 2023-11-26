@echo off
cd glsl
glslc -fshader-stage=compute measure_cs.glsl -o ..\measure_cs.spirv
glslc -fshader-stage=compute depth_occlusion_cs.glsl -o ..\depth_occlusion_cs.spirv
glslc -fshader-stage=compute lighting_cs.glsl -o ..\lighting_cs.spirv
glslc -fshader-stage=vertex draw_equirect_vs.glsl -o ..\draw_equirect_vs.spirv
glslc -fshader-stage=fragment draw_equirect_fs.glsl -o ..\draw_equirect_fs.spirv
cd ..
