use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uchar, c_uint};
use std::path::Path;
use std::result::Result;

/// Tunnel file descriptor type (the C API takes an `int` on every platform).
#[cfg(unix)]
pub type RawFd = std::os::unix::io::RawFd;
/// Tunnel file descriptor type (the C API takes an `int` on every platform).
#[cfg(windows)]
pub type RawFd = c_int;

#[cfg(unix)]
fn path_to_cstring(path: &Path) -> CString {
    use std::os::unix::ffi::OsStrExt;
    CString::new(path.as_os_str().as_bytes()).unwrap()
}

#[cfg(windows)]
fn path_to_cstring(path: &Path) -> CString {
    CString::new(path.to_string_lossy().as_bytes()).unwrap()
}

unsafe extern "C" {
    fn hev_socks5_tunnel_main_from_file(config_path: *const c_char, tun_fd: c_int) -> c_int;
}

unsafe extern "C" {
    fn hev_socks5_tunnel_main_from_str(
        config_str: *const c_uchar,
        config_len: c_uint,
        tun_fd: c_int,
    ) -> c_int;
}

unsafe extern "C" {
    fn hev_socks5_tunnel_quit();
}

unsafe extern "C" {
    fn hev_socks5_tunnel_stats(
        tx_packets: *mut usize,
        tx_bytes: *mut usize,
        rx_packets: *mut usize,
        rx_bytes: *mut usize,
    );
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
    let path = path_to_cstring(config_path);
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

/// Tunnel interface traffic statistics.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Stats {
    pub tx_packets: usize,
    pub tx_bytes: usize,
    pub rx_packets: usize,
    pub rx_bytes: usize,
}

/// Retrieve tunnel interface traffic statistics.
pub fn stats() -> Stats {
    let mut s = Stats::default();
    unsafe {
        hev_socks5_tunnel_stats(
            &mut s.tx_packets,
            &mut s.tx_bytes,
            &mut s.rx_packets,
            &mut s.rx_bytes,
        );
    }
    s
}

#[cfg(test)]
mod tests {
    #[test]
    fn stats_idle_is_zero() {
        assert_eq!(super::stats(), super::Stats::default());
    }
}
