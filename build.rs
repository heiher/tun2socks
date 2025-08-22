use std::process::Command;

fn main() {
    Command::new("make")
        .arg("static")
        .arg("-j8")
        .current_dir("impl")
        .status()
        .expect("Failed to build impl");

    println!("cargo:rustc-link-search=native=impl/bin");
    println!("cargo:rustc-link-lib=static=hev-socks5-tunnel");
    println!("cargo:rustc-link-search=native=impl/third-part/hev-task-system/bin");
    println!("cargo:rustc-link-lib=static=hev-task-system");
    println!("cargo:rustc-link-search=native=impl/third-part/lwip/bin");
    println!("cargo:rustc-link-lib=static=lwip");
    println!("cargo:rustc-link-search=native=impl/third-part/yaml/bin");
    println!("cargo:rustc-link-lib=static=yaml");
}
