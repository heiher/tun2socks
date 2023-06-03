use std::os::raw::{c_char, c_int};

extern "C" {
    fn hev_socks5_tunnel_main(config_path: *const c_char, tun_fd: c_int) -> c_int;
}

extern "C" {
    fn hev_socks5_tunnel_quit();
}

/// Start and run the socks5 tunnel, this function will blocks until the
/// quit() is called or an error occurs.
///
/// # Arguments
///
/// * `config_path` - config file path
/// * `tun_fd` - tunnel file descriptor
///
/// # Returns
///
/// Returns zero on successful, otherwise returns -1.
pub fn main(config_path: *const c_char, tun_fd: c_int) -> c_int {
    unsafe { hev_socks5_tunnel_main(config_path, tun_fd) }
}

/// Stop the socks5 tunnel.
pub fn quit() {
    unsafe {
        hev_socks5_tunnel_quit();
    }
}
