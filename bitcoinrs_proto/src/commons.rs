#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::net::{IpAddr, SocketAddr};
use std::time::{SystemTime, UNIX_EPOCH};

use bitcoinrs_bytes::Bytes;

pub const SRV_NODE_NETWORK: u64 = 1;
pub const SRV_NODE_GETUTXO: u64 = 2;
pub const SRV_NODE_BLOOM: u64 = 4;
pub const SRV_NODE_WITNESS: u64 = 8;
pub const SRV_NODE_NETWORK_LIMITED: u64 = 1024;

pub struct Services(pub u64);

impl Services {
    pub const NETWORK: Services = Services(SRV_NODE_NETWORK);
    pub const GETUTXO: Services = Services(SRV_NODE_GETUTXO);
    pub const BLOOM: Services = Services(SRV_NODE_BLOOM);
    pub const WITNESS: Services = Services(SRV_NODE_WITNESS);
    pub const NETWORK_LIMITED: Services = Services(SRV_NODE_NETWORK_LIMITED);

    pub fn empty() -> Services {
        Services(0)
    }

    pub fn add_network(&mut self) -> &mut Self {
        self.0 |= SRV_NODE_NETWORK;
        self
    }

    pub fn add_getutxo(&mut self) -> &mut Self {
        self.0 |= SRV_NODE_GETUTXO;
        self
    }

    pub fn add_bloom(&mut self) -> &mut Self {
        self.0 |= SRV_NODE_BLOOM;
        self
    }

    pub fn add_witness(&mut self) -> &mut Self {
        self.0 |= SRV_NODE_WITNESS;
        self
    }

    pub fn add_network_limited(&mut self) -> &mut Self {
        self.0 |= SRV_NODE_NETWORK_LIMITED;
        self
    }
}

impl Bytes for Services {
    fn length(&self) -> usize {
        8
    }

    fn write_to(&self, buf: &mut Vec<u8>) {
        self.0.to_le().write_to(buf);
    }
}

pub struct NetAddr {
    time: SystemTime, // Not present in version message.
    services: Services,
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
        self.services.write_to(buf);

        write_addr(self.addr, buf);
    }
}

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

impl Bytes for NetAddrForVersionMsg {
    fn length(&self) -> usize {
        26
    }

    fn write_to(&self, buf: &mut Vec<u8>) {
        self.services.write_to(buf);
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
