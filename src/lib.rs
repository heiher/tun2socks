use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::RawFd;
use std::path::Path;
use std::result::Result;

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
pub fn main(config_path: &Path, tun_fd: RawFd) -> Result<(), i32> {
    let path = CString::new(config_path.as_os_str().as_bytes()).unwrap();
    let res;

    unsafe {
        res = hev_socks5_tunnel_main(path.as_ptr() as *const i8, tun_fd);
    }

    match res {
        0 => Ok(()),
        r => Err(r),
    }
}

/// Stop the socks5 tunnel.
pub fn quit() {
    unsafe {
        hev_socks5_tunnel_quit();
    }
}
