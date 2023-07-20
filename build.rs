use std::env;

pub enum System {
    Linux,
    Windows,
    Android,
}

fn main() {
    println!("cargo:rustc-cfg=build={:?}",env::var("PROFILE").unwrap());

#[cfg(target_os="linux")]
    let (system,system_name) = (System::Linux,"linux");
#[cfg(target_os="windows")]
    let (system,system_name) = (System::Windows,"windows");
#[cfg(target_os="android")]
    let (system,system_name) = (System::Android,"android");

    println!("cargo:rustc-cfg=system=\"{}\"",system_name);

    match system {

        System::Linux => {

            println!("cargo:rustc-link-lib=X11");
            println!("cargo:rustc-link-lib=X11-xcb");
            println!("cargo:rustc-link-lib=xcb");
        },

        System::Windows => {

        },

        System::Android => {

        },
    }

    println!("cargo:rustc-link-lib=vulkan");

    println!("cargo:rerun-if-changed=build.rs");
}
