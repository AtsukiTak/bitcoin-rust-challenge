#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::net::{IpAddr, SocketAddr};
use std::time::{SystemTime, UNIX_EPOCH};

use bitcoinrs_bytes::{Encodable, EncodableSized, WriteBuf, endian::{u16_b, u16_l, u32_l, u64_l}};

#[derive(Debug, Clone, Copy)]
pub struct Timestamp(u64);

impl Timestamp {
    pub fn now() -> Timestamp {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Timestamp(ts)
    }
}

impl EncodableSized for Timestamp {
    const SIZE: usize = 8;
    type Array = [u8; 8];

    fn bytes(&self) -> [u8; 8] {
        u64_l::new(self.0).bytes()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CompactSize(pub u64);

impl Encodable for CompactSize {
    fn length(&self) -> usize {
        if self.0 < 0xFD {
            1
        } else if self.0 <= 0xFFFF {
            3
        } else if self.0 <= 0xFFFF_FFFF {
            5
        } else {
            9
        }
    }

    fn encode<W: WriteBuf>(&self, buf: &mut W) {
        if self.0 < 0xFD {
            buf.write_bytes(&[self.0 as u8]);
        } else if self.0 <= 0xFFFF {
            buf.write_bytes(&[0xFD]);
            u16_l::new(self.0 as u16).encode(buf);
        } else if self.0 <= 0xFFFF_FFFF {
            buf.write_bytes(&[0xFE]);
            u32_l::new(self.0 as u32).encode(buf);
        } else {
            buf.write_bytes(&[0xFF]);
            u64_l::new(self.0).encode(buf);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VarStr<'a>(pub &'a str);

impl<'a> Encodable for VarStr<'a> {
    fn length(&self) -> usize {
        CompactSize(self.0.len() as u64).length() + self.0.len()
    }

    fn encode<W: WriteBuf>(&self, buf: &mut W) {
        CompactSize(self.0.len() as u64).encode(buf);
        buf.write_bytes(self.0.as_bytes());
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Service {
    Network = 1,
    Getutxo = 2,
    Bloom = 4,
    Witness = 8,
    NetworkLimited = 1024,
}

#[derive(Debug, Clone, Copy)]
pub struct Services(u64);

impl Services {
    pub fn empty() -> Services {
        Services(0)
    }

    pub fn new(services: &[Service]) -> Services {
        let mut srvs = Services::empty();
        for service in services.iter() {
            srvs.add(*service);
        }
        srvs
    }

    pub fn add(&mut self, service: Service) {
        self.0 |= service as u64;
    }
}

impl EncodableSized for Services {
    const SIZE: usize = 8;
    type Array = [u8; 8];

    fn bytes(&self) -> [u8; 8] {
        u64_l::new(self.0).bytes()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NetAddr {
    ts: Timestamp, // Not present in version message.
    services: Services,
    addr: SocketAddr,
}

impl NetAddr {
    pub fn new(ts: Timestamp, services: Services, addr: SocketAddr) -> NetAddr {
        NetAddr {
            ts: ts,
            services: services,
            addr: addr,
        }
    }
}

impl EncodableSized for NetAddr {
    const SIZE: usize = 30;
    type Array = [u8; 30];

    fn bytes(&self) -> [u8; 30] {
        let mut buf = [0; 30];

        // Write time field
        (&mut buf[0..4]).copy_from_slice(&self.ts.bytes());

        // Write services field
        (&mut buf[4..12]).copy_from_slice(&self.services.bytes());

        write_addr(self.addr, &mut buf[12..30]);

        buf
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NetAddrForVersionMsg {
    services: Services,
    addr: SocketAddr,
}

impl NetAddrForVersionMsg {
    pub fn new(services: Services, addr: SocketAddr) -> NetAddrForVersionMsg {
        NetAddrForVersionMsg {
            services: services,
            addr: addr,
        }
    }
}

impl EncodableSized for NetAddrForVersionMsg {
    const SIZE: usize = 26;
    type Array = [u8; 26];

    fn bytes(&self) -> [u8; 26] {
        let mut buf = [0; 26];
        (&mut buf[0..8]).copy_from_slice(&self.services.bytes());
        write_addr(self.addr, &mut buf[8..26]);
        buf
    }
}

fn write_addr(addr: SocketAddr, buf: &mut [u8]) {
    let ipv6 = match addr.ip() {
        IpAddr::V4(v4) => v4.to_ipv6_mapped(),
        IpAddr::V6(v6) => v6,
    };
    (&mut buf[0..16]).copy_from_slice(&ipv6.octets());

    (&mut buf[16..18]).copy_from_slice(&u16_b::new(addr.port()).bytes());
}
