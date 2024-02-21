use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uchar, c_uint};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::RawFd;
use std::path::Path;
use std::result::Result;

extern "C" {
    fn hev_socks5_tunnel_main_from_file(config_path: *const c_char, tun_fd: c_int) -> c_int;
}

extern "C" {
    fn hev_socks5_tunnel_main_from_str(
        config_str: *const c_uchar,
        config_len: c_uint,
        tun_fd: c_int,
    ) -> c_int;
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
/// Returns Ok(()) on successful, otherwise returns Err(r).
pub fn main_from_file(config_path: &Path, tun_fd: RawFd) -> Result<(), i32> {
    let path = CString::new(config_path.as_os_str().as_bytes()).unwrap();
    let res = unsafe { hev_socks5_tunnel_main_from_file(path.as_ptr(), tun_fd) };

    match res {
        0 => Ok(()),
        r => Err(r),
    }
}

/// Start and run the socks5 tunnel, this function will blocks until the
/// quit() is called or an error occurs.
///
/// # Arguments
///
/// * `config_str` - config string
/// * `tun_fd` - tunnel file descriptor
///
/// # Returns
///
/// Returns Ok(()) on successful, otherwise returns Err(r).
pub fn main_from_str(config_str: &str, tun_fd: RawFd) -> Result<(), i32> {
    let config_bytes = config_str.as_bytes();
    let res = unsafe {
        hev_socks5_tunnel_main_from_str(config_bytes.as_ptr(), config_bytes.len() as c_uint, tun_fd)
    };

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
