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
}
