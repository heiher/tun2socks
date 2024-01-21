extern crate cc;

const INCLUDES: &[&str] = &[
    "impl/include",
    "impl/src/misc",
    "impl/src/core/include",
    "impl/third-part/hev-task-system/include",
    "impl/third-part/hev-task-system/src",
    "impl/third-part/lwip/include",
    "impl/third-part/lwip/include/ports/unix",
    "impl/third-part/yaml/include",
];

const SOURCES: &[&str] = &[
    "impl/src/core/src/hev-socks5-logger.c",
    "impl/src/core/src/hev-socks5-tcp.c",
    "impl/src/core/src/hev-rbtree.c",
    "impl/src/core/src/hev-socks5-misc.c",
    "impl/src/core/src/hev-socks5-authenticator.c",
    "impl/src/core/src/hev-socks5-client-tcp.c",
    "impl/src/core/src/hev-socks5-client-udp.c",
    "impl/src/core/src/hev-socks5-client.c",
    "impl/src/core/src/hev-socks5-server.c",
    "impl/src/core/src/hev-socks5-udp.c",
    "impl/src/core/src/hev-socks5-user.c",
    "impl/src/core/src/hev-socks5.c",
    "impl/src/misc/hev-list.c",
    "impl/src/misc/hev-logger.c",
    "impl/src/misc/hev-ring-buffer.c",
    "impl/src/misc/hev-exec.c",
    "impl/src/misc/hev-utils.c",
    "impl/src/hev-config.c",
    "impl/src/hev-jni.c",
    "impl/src/hev-main.c",
    "impl/src/hev-socks5-session-tcp.c",
    "impl/src/hev-socks5-session-udp.c",
    "impl/src/hev-socks5-session.c",
    "impl/src/hev-socks5-tunnel.c",
    "impl/src/hev-tunnel-freebsd.c",
    "impl/src/hev-tunnel-linux.c",
    "impl/src/hev-tunnel-macos.c",
    "impl/third-part/hev-task-system/src/kern/aide/hev-task-aide.c",
    "impl/third-part/hev-task-system/src/kern/core/hev-task-system-schedule.c",
    "impl/third-part/hev-task-system/src/kern/core/hev-task-system.c",
    "impl/third-part/hev-task-system/src/kern/io/hev-task-io-reactor-epoll.c",
    "impl/third-part/hev-task-system/src/kern/io/hev-task-io-reactor-kqueue.c",
    "impl/third-part/hev-task-system/src/kern/io/hev-task-io-reactor.c",
    "impl/third-part/hev-task-system/src/kern/itc/hev-task-channel-select.c",
    "impl/third-part/hev-task-system/src/kern/itc/hev-task-channel.c",
    "impl/third-part/hev-task-system/src/kern/sync/hev-task-cond.c",
    "impl/third-part/hev-task-system/src/kern/sync/hev-task-mutex.c",
    "impl/third-part/hev-task-system/src/kern/task/hev-task-call.c",
    "impl/third-part/hev-task-system/src/kern/task/hev-task-stack-heap.c",
    "impl/third-part/hev-task-system/src/kern/task/hev-task-stack-mmap.c",
    "impl/third-part/hev-task-system/src/kern/task/hev-task-executer.c",
    "impl/third-part/hev-task-system/src/kern/task/hev-task.c",
    "impl/third-part/hev-task-system/src/kern/time/hev-task-timer-kevent.c",
    "impl/third-part/hev-task-system/src/kern/time/hev-task-timer-timerfd.c",
    "impl/third-part/hev-task-system/src/kern/time/hev-task-timer.c",
    "impl/third-part/hev-task-system/src/lib/dns/hev-task-dns-proxy.c",
    "impl/third-part/hev-task-system/src/lib/dns/hev-task-dns.c",
    "impl/third-part/hev-task-system/src/lib/io/basic/hev-task-io.c",
    "impl/third-part/hev-task-system/src/lib/io/buffer/hev-circular-buffer.c",
    "impl/third-part/hev-task-system/src/lib/io/pipe/hev-task-io-pipe.c",
    "impl/third-part/hev-task-system/src/lib/io/poll/hev-task-io-poll.c",
    "impl/third-part/hev-task-system/src/lib/io/socket/hev-task-io-socket.c",
    "impl/third-part/hev-task-system/src/lib/list/hev-list.c",
    "impl/third-part/hev-task-system/src/lib/misc/hev-debugger.c",
    "impl/third-part/hev-task-system/src/lib/rbtree/hev-rbtree-cached.c",
    "impl/third-part/hev-task-system/src/lib/rbtree/hev-rbtree.c",
    "impl/third-part/hev-task-system/src/lib/object/hev-object-atomic.c",
    "impl/third-part/hev-task-system/src/lib/object/hev-object.c",
    "impl/third-part/hev-task-system/src/mem/api/hev-memory-allocator-api.c",
    "impl/third-part/hev-task-system/src/mem/base/hev-memory-allocator.c",
    "impl/third-part/hev-task-system/src/mem/simple/hev-memory-allocator-simple.c",
    "impl/third-part/hev-task-system/src/mem/slice/hev-memory-allocator-slice.c",
    "impl/third-part/lwip/src/api/err.c",
    "impl/third-part/lwip/src/api/netifapi.c",
    "impl/third-part/lwip/src/api/api_lib.c",
    "impl/third-part/lwip/src/api/api_msg.c",
    "impl/third-part/lwip/src/api/if_api.c",
    "impl/third-part/lwip/src/api/netbuf.c",
    "impl/third-part/lwip/src/api/netdb.c",
    "impl/third-part/lwip/src/api/sockets.c",
    "impl/third-part/lwip/src/api/tcpip.c",
    "impl/third-part/lwip/src/core/ip.c",
    "impl/third-part/lwip/src/core/ipv4/acd.c",
    "impl/third-part/lwip/src/core/ipv4/autoip.c",
    "impl/third-part/lwip/src/core/ipv4/dhcp.c",
    "impl/third-part/lwip/src/core/ipv4/etharp.c",
    "impl/third-part/lwip/src/core/ipv4/icmp.c",
    "impl/third-part/lwip/src/core/ipv4/igmp.c",
    "impl/third-part/lwip/src/core/ipv4/ip4.c",
    "impl/third-part/lwip/src/core/ipv4/ip4_addr.c",
    "impl/third-part/lwip/src/core/ipv4/ip4_frag.c",
    "impl/third-part/lwip/src/core/ipv6/ethip6.c",
    "impl/third-part/lwip/src/core/ipv6/inet6.c",
    "impl/third-part/lwip/src/core/ipv6/dhcp6.c",
    "impl/third-part/lwip/src/core/ipv6/icmp6.c",
    "impl/third-part/lwip/src/core/ipv6/ip6.c",
    "impl/third-part/lwip/src/core/ipv6/ip6_addr.c",
    "impl/third-part/lwip/src/core/ipv6/ip6_frag.c",
    "impl/third-part/lwip/src/core/ipv6/mld6.c",
    "impl/third-part/lwip/src/core/ipv6/nd6.c",
    "impl/third-part/lwip/src/core/memp.c",
    "impl/third-part/lwip/src/core/altcp.c",
    "impl/third-part/lwip/src/core/altcp_alloc.c",
    "impl/third-part/lwip/src/core/altcp_tcp.c",
    "impl/third-part/lwip/src/core/def.c",
    "impl/third-part/lwip/src/core/dns.c",
    "impl/third-part/lwip/src/core/inet_chksum.c",
    "impl/third-part/lwip/src/core/init.c",
    "impl/third-part/lwip/src/core/mem.c",
    "impl/third-part/lwip/src/core/netif.c",
    "impl/third-part/lwip/src/core/pbuf.c",
    "impl/third-part/lwip/src/core/raw.c",
    "impl/third-part/lwip/src/core/stats.c",
    "impl/third-part/lwip/src/core/sys.c",
    "impl/third-part/lwip/src/core/tcp.c",
    "impl/third-part/lwip/src/core/tcp_in.c",
    "impl/third-part/lwip/src/core/tcp_out.c",
    "impl/third-part/lwip/src/core/timeouts.c",
    "impl/third-part/lwip/src/core/udp.c",
    "impl/third-part/lwip/src/netif/ppp/chap-md5.c",
    "impl/third-part/lwip/src/netif/ppp/ecp.c",
    "impl/third-part/lwip/src/netif/ppp/eui64.c",
    "impl/third-part/lwip/src/netif/ppp/mppe.c",
    "impl/third-part/lwip/src/netif/ppp/polarssl/arc4.c",
    "impl/third-part/lwip/src/netif/ppp/polarssl/des.c",
    "impl/third-part/lwip/src/netif/ppp/polarssl/md4.c",
    "impl/third-part/lwip/src/netif/ppp/polarssl/md5.c",
    "impl/third-part/lwip/src/netif/ppp/polarssl/sha1.c",
    "impl/third-part/lwip/src/netif/ppp/pppapi.c",
    "impl/third-part/lwip/src/netif/ppp/pppcrypt.c",
    "impl/third-part/lwip/src/netif/ppp/auth.c",
    "impl/third-part/lwip/src/netif/ppp/ccp.c",
    "impl/third-part/lwip/src/netif/ppp/chap-new.c",
    "impl/third-part/lwip/src/netif/ppp/chap_ms.c",
    "impl/third-part/lwip/src/netif/ppp/demand.c",
    "impl/third-part/lwip/src/netif/ppp/eap.c",
    "impl/third-part/lwip/src/netif/ppp/fsm.c",
    "impl/third-part/lwip/src/netif/ppp/ipcp.c",
    "impl/third-part/lwip/src/netif/ppp/ipv6cp.c",
    "impl/third-part/lwip/src/netif/ppp/lcp.c",
    "impl/third-part/lwip/src/netif/ppp/magic.c",
    "impl/third-part/lwip/src/netif/ppp/multilink.c",
    "impl/third-part/lwip/src/netif/ppp/ppp.c",
    "impl/third-part/lwip/src/netif/ppp/pppoe.c",
    "impl/third-part/lwip/src/netif/ppp/pppol2tp.c",
    "impl/third-part/lwip/src/netif/ppp/pppos.c",
    "impl/third-part/lwip/src/netif/ppp/upap.c",
    "impl/third-part/lwip/src/netif/ppp/utils.c",
    "impl/third-part/lwip/src/netif/ppp/vj.c",
    "impl/third-part/lwip/src/netif/zepif.c",
    "impl/third-part/lwip/src/netif/bridgeif.c",
    "impl/third-part/lwip/src/netif/bridgeif_fdb.c",
    "impl/third-part/lwip/src/netif/ethernet.c",
    "impl/third-part/lwip/src/netif/lowpan6.c",
    "impl/third-part/lwip/src/netif/lowpan6_ble.c",
    "impl/third-part/lwip/src/netif/lowpan6_common.c",
    "impl/third-part/lwip/src/netif/slipif.c",
    "impl/third-part/lwip/src/ports/unix/lib/mem.c",
    "impl/third-part/lwip/src/ports/unix/port/netif/pcapif.c",
    "impl/third-part/lwip/src/ports/unix/port/netif/tapif.c",
    "impl/third-part/lwip/src/ports/unix/port/netif/fifo.c",
    "impl/third-part/lwip/src/ports/unix/port/netif/list.c",
    "impl/third-part/lwip/src/ports/unix/port/netif/sio.c",
    "impl/third-part/lwip/src/ports/unix/port/perf.c",
    "impl/third-part/lwip/src/ports/unix/port/sys_arch.c",
    "impl/third-part/yaml/src/api.c",
    "impl/third-part/yaml/src/dumper.c",
    "impl/third-part/yaml/src/emitter.c",
    "impl/third-part/yaml/src/loader.c",
    "impl/third-part/yaml/src/parser.c",
    "impl/third-part/yaml/src/reader.c",
    "impl/third-part/yaml/src/scanner.c",
    "impl/third-part/yaml/src/writer.c",
];

fn main() {
    let mut build = cc::Build::new();

    // hev-socks5-tunnel
    build.define("COMMIT_ID", "\"\"");

    // hev-task-system
    build.define("ENABLE_PTHREAD", None);
    build.define("ENABLE_STACK_OVERFLOW_DETECTION", None);
    build.define("ENABLE_MEMALLOC_SLICE", None);
    build.define("ENABLE_IO_SPLICE_SYSCALL", None);
    build.define("CONFIG_STACK_BACKEND", "STACK_MMAP");
    build.define("CONFIG_STACK_OVERFLOW_DETECTION", "1");
    build.define("CONFIG_MEMALLOC_SLICE_ALIGN", "64");
    build.define("CONFIG_MEMALLOC_SLICE_MAX_SIZE", "4096");
    build.define("CONFIG_MEMALLOC_SLICE_MAX_COUNT", "1000");
    build.define("CONFIG_SCHED_CLOCK", "CLOCK_NONE");

    // yaml
    build.define("YAML_VERSION_MAJOR", "0");
    build.define("YAML_VERSION_MINOR", "2");
    build.define("YAML_VERSION_PATCH", "2");
    build.define("YAML_VERSION_STRING", "\"0.2.2\"");

    for include in INCLUDES {
        build.include(include);
    }

    for source in SOURCES {
        build.file(source);
    }

    build.warnings(false);
    build.compile("hev-socks5-tunnel");
}
