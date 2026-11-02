use std::process::Command;

fn main() {
    let windows = std::env::var("CARGO_CFG_TARGET_OS").is_ok_and(|os| os == "windows");

    let mut make = Command::new("make");
    make.arg("static").arg("-j8").current_dir("impl");
    if windows {
        let arch = match std::env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
            Ok("x86") => "x86",
            Ok("aarch64") => "arm64",
            _ => "x64",
        };
        make.arg(format!("ARCH={arch}"));
    }
    let status = make.status().expect("Failed to run make for impl");
    assert!(status.success(), "Failed to build impl");

    println!("cargo:rustc-link-search=native=impl/bin");
    println!("cargo:rustc-link-lib=static=hev-socks5-tunnel");
    println!("cargo:rustc-link-search=native=impl/third-part/hev-task-system/bin");
    println!("cargo:rustc-link-lib=static=hev-task-system");
    println!("cargo:rustc-link-search=native=impl/third-part/lwip/bin");
    println!("cargo:rustc-link-lib=static=lwip");
    println!("cargo:rustc-link-search=native=impl/third-part/yaml/bin");
    println!("cargo:rustc-link-lib=static=yaml");

    if windows {
        for lib in ["ws2_32", "iphlpapi", "advapi32", "kernel32", "oldnames"] {
            println!("cargo:rustc-link-lib={lib}");
        }
    }
}
