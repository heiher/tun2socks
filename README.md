# Tun2Socks

Rust bindings for [HevSocks5Tunnel](https://github.com/heiher/hev-socks5-tunnel)

## API

```rust
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
pub fn main(config_path: *const c_char, tun_fd: c_int) -> c_int;

/// Stop the socks5 tunnel.
pub fn quit();
```

## Contributors

* **hev** - https://hev.cc

## License

MIT
