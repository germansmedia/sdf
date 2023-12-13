// SDF - render fractals in VR
// by Desmond Germans, 2023

use crate::*;

#[derive(Clone,Copy,Debug,PartialEq)]
#[repr(C)]
pub struct Params {

    pub pose: Mat4x4<f32>,  // camera pose inside fractal space

    pub forward_dir: Vec4<f32>,  // measurement direction vector
    pub key_light_pos: Vec4<f32>,  // key light position
    pub key_light_color: Vec4<f32>,  // key light color
    pub shadow_power: Vec4<f32>,  // multicolor shadow power

    pub sky_light_color: Vec4<f32>,  // sky light color
    pub ambient_light_color: Vec4<f32>,  // ambient light color
    pub background_color: Vec4<f32>,  // background color
    pub glow_color: Vec4<f32>,  // glow color

    pub palette: [Vec4<f32>; 8],  // discrete palette

    pub scale: f32,  // The Scale
    pub horizon: f32,  // unscaled furthest distance
    pub escape: f32,  // fractal escape value
    pub dtf_const: f32,  // distance to fractal const

    pub dtf_linear: f32,  // distance to fractal linear
    pub max_steps: u32,  // maximum number of ray marching steps
    pub max_iterations: u32,  // maximum number of iterations
    pub step_size: f32,  // size of marching step

    pub iod: f32,  // unscaled inter-ocular distance
    pub tbd0: u32,
    pub tbd1: u32,
    pub tbd2: u32,
}
