#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::net::{IpAddr, SocketAddr};

use bitcoinrs_bytes::Bytes;

pub struct NetAddr {
    time: Option<u32>, // Not present in version message.
    services: u64,
    addr: SocketAddr,
}

impl Bytes for NetAddr {
    fn length(&self) -> usize {
        if self.time.is_some() {
            30
        } else {
            26
        }
    }

    fn write_to(&self, buf: &mut Vec<u8>) {
        // Write time field
        if let Some(time) = self.time {
            time.to_le().write_to(buf);
        }

        // Write services field
        self.services.to_le().write_to(buf);

        // Write ipv6 field
        let ipv6 = match self.addr.ip() {
            IpAddr::V4(v4) => v4.to_ipv6_mapped(),
            IpAddr::V6(v6) => v6,
        };
        buf.extend_from_slice(&ipv6.octets());

        // Write port field
        self.addr.port().to_be().write_to(buf);
    }
}
