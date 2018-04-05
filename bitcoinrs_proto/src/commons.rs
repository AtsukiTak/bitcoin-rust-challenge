#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::net::{IpAddr, SocketAddr};
use std::time::{SystemTime, UNIX_EPOCH};

use bitcoinrs_bytes::Bytes;

pub struct NetAddr {
    time: SystemTime, // Not present in version message.
    services: u64,
    addr: SocketAddr,
}

impl Bytes for NetAddr {
    fn length(&self) -> usize {
        30
    }

    fn write_to(&self, buf: &mut Vec<u8>) {
        // Write time field
        self.time
            .duration_since(UNIX_EPOCH)
            .unwrap() // `Duration` since `UNIX_EPOCH`
            .as_secs()
            .to_le()
            .write_to(buf);

        // Write services field
        self.services.to_le().write_to(buf);

        write_addr(self.addr, buf);
    }
}

pub struct NetAddrForVersionMsg {
    services: u64,
    addr: SocketAddr,
}

impl Bytes for NetAddrForVersionMsg {
    fn length(&self) -> usize {
        26
    }

    fn write_to(&self, buf: &mut Vec<u8>) {
        self.services.to_le().write_to(buf);
        write_addr(self.addr, buf);
    }
}

fn write_addr(addr: SocketAddr, buf: &mut Vec<u8>) {
    let ipv6 = match addr.ip() {
        IpAddr::V4(v4) => v4.to_ipv6_mapped(),
        IpAddr::V6(v6) => v6,
    };
    buf.extend_from_slice(&ipv6.octets());

    addr.port().to_be().write_to(buf);
}
