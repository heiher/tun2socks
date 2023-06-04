# Tun2Socks

Rust bindings for [HevSocks5Tunnel](https://github.com/heiher/hev-socks5-tunnel)

## Features

* IPv4/IPv6. (dual stack)
* Redirect TCP connections.
* Redirect UDP packets. (Fullcone NAT, UDP in UDP/TCP)
* Linux/Android/FreeBSD/macOS/iOS.

## Benchmarks

See [here](https://github.com/heiher/hev-socks5-tunnel/wiki/Benchmarks) for more details.

### Speed

![](https://github.com/heiher/hev-socks5-tunnel/wiki/res/upload-speed.png)
![](https://github.com/heiher/hev-socks5-tunnel/wiki/res/download-speed.png)

### CPU usage

![](https://github.com/heiher/hev-socks5-tunnel/wiki/res/upload-cpu.png)
![](https://github.com/heiher/hev-socks5-tunnel/wiki/res/download-cpu.png)

### Memory usage

![](https://github.com/heiher/hev-socks5-tunnel/wiki/res/upload-mem.png)
![](https://github.com/heiher/hev-socks5-tunnel/wiki/res/download-mem.png)

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
