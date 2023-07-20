#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
//#![allow(dead_code)]

#[cfg(system="linux")]
mod linux_full;
#[cfg(system="linux")]
pub use linux_full::*;

#[cfg(system="windows")]
mod windows;
#[cfg(system="windows")]
pub use windows::*;

#[cfg(system="android")]
mod android;
#[cfg(system="android")]
pub use android::*;
