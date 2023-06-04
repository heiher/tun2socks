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
/// Returns Ok(()) on successful, otherwise returns non-zero.
pub fn main(config_path: &Path, tun_fd: RawFd) -> Result<(), i32>

/// Stop the socks5 tunnel.
pub fn quit()
```

## Contributors

* **hev** - https://hev.cc

## License

MIT
