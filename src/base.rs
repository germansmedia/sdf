// SDF - render fractals in VR
// by Desmond Germans, 2023

use crate::*;

#[derive(Clone,Copy,Debug,PartialEq)]
#[repr(C)]
pub struct March {

    pub pose: Mat4x4<f32>,  // pose inside fractal space

    pub scale: f32,  // scale factor
    pub horizon: f32,  // unscaled furthest horizon
    pub escape: f32,  // escape value
    pub de_stop: f32,  // unscaled MB3D de_stop

    pub de_stop_factor: f32,  // unscaled MB3D de_stop_factor
    pub max_steps: u32,  // maximum number of marching steps
    pub max_iterations: u32,  // maximum number of iterations
    pub iod: f32,  // unscaled distance between left and right eyes

    pub forward_dir: Vec4<f32>,  // view direction (distance measurement only)
}

#[derive(Clone,Copy,Debug,PartialEq)]
#[repr(C)]
pub struct Render {
    pub key_light_pos: Vec4<f32>,
    pub key_light_color: Vec4<f32>,
    pub shadow_power: Vec4<f32>,
    pub sky_light_color: Vec4<f32>,

    pub ambient_light_color: Vec4<f32>,
    pub background_color: Vec4<f32>,
    pub glow_color: Vec4<f32>,
    pub tbd0: Vec4<f32>,

    pub palette: [Vec4<f32>; 4],
}
