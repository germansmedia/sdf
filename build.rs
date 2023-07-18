fn main() {
    println!("cargo:rustc-cfg=build={:?}",std::env::var("PROFILE").unwrap());
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=X11-xcb");
    println!("cargo:rustc-link-lib=xcb");
    println!("cargo:rustc-link-lib=vulkan");
    println!("cargo:rerun-if-changed=build.rs");
}
