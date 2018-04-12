#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::net::{IpAddr, SocketAddr, Ipv6Addr};
use std::time::{SystemTime, UNIX_EPOCH};

use bitcoinrs_bytes::{Decodable, Encodable, EncodableSized, ReadBuf, ReadError, WriteBuf,
                      endian::{u16_b, u16_l, u32_l, u64_l}};

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

impl Decodable for Timestamp {
    fn decode<R: ReadBuf>(bytes: &mut R) -> Result<Timestamp, ReadError> {
        Ok(Timestamp(bytes.read::<u64_l>()?.value()))
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
            buf.write(u16_l::new(self.0 as u16));
        } else if self.0 <= 0xFFFF_FFFF {
            buf.write_bytes(&[0xFE]);
            buf.write(u32_l::new(self.0 as u32));
        } else {
            buf.write_bytes(&[0xFF]);
            buf.write(u64_l::new(self.0));
        }
    }
}

impl Decodable for CompactSize {
    fn decode<R: ReadBuf>(bytes: &mut R) -> Result<CompactSize, ReadError> {
        let first = bytes.read::<u8>()?;
        if first < 0xFD {
            Ok(CompactSize(first as u64))
        } else if first == 0xFD {
            Ok(CompactSize(bytes.read::<u16_l>()?.value() as u64))
        } else if first == 0xFE {
            Ok(CompactSize(bytes.read::<u32_l>()?.value() as u64))
        } else {
            Ok(CompactSize(bytes.read::<u64_l>()?.value()))
        }
    }
}

#[derive(Debug, Clone)]
pub struct VarStr(pub String);

impl<'a> Encodable for &'a VarStr {
    fn length(&self) -> usize {
        CompactSize(self.0.len() as u64).length() + self.0.len()
    }

    fn encode<W: WriteBuf>(&self, buf: &mut W) {
        buf.write(CompactSize(self.0.len() as u64));
        buf.write_bytes(self.0.as_bytes());
    }
}

impl Decodable for VarStr {
    fn decode<R: ReadBuf>(bytes: &mut R) -> Result<VarStr, ReadError> {
        let len = bytes.read::<CompactSize>()?.0;
        let s = {
            let mut buf = Vec::with_capacity(len as usize);
            buf.write_zeros(len as usize);
            bytes.read_bytes(buf.as_mut_slice())?;
            String::from_utf8(buf).map_err(|_| ReadError::InvalidBytes)?
        };
        Ok(VarStr(s))
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

impl Decodable for Services {
    fn decode<R: ReadBuf>(bytes: &mut R) -> Result<Services, ReadError> {
        Ok(Services(bytes.read::<u64_l>()?.value()))
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

impl Decodable for NetAddr {
    fn decode<R: ReadBuf>(bytes: &mut R) -> Result<NetAddr, ReadError> {
        let ts = bytes.read::<Timestamp>()?;
        let services = bytes.read::<Services>()?;
        let socket_addr = read_addr(bytes)?;
        Ok(NetAddr {
            ts: ts,
            services: services,
            addr: socket_addr,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NetAddrForVersionMsg {
    pub services: Services,
    pub addr: SocketAddr,
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

impl Decodable for NetAddrForVersionMsg {
    fn decode<R: ReadBuf>(bytes: &mut R) -> Result<NetAddrForVersionMsg, ReadError> {
        let services = bytes.read::<Services>()?;
        let socket_addr = read_addr(bytes)?;
        Ok(NetAddrForVersionMsg {
            services: services,
            addr: socket_addr,
        })
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

fn read_addr<R: ReadBuf>(bytes: &mut R) -> Result<SocketAddr, ReadError> {
    let mut ip_octet = [0; 16];
    bytes.read_bytes(&mut ip_octet)?;
    let ipv6 = Ipv6Addr::from(ip_octet);
    let port = bytes.read::<u16_l>()?.value();
    Ok(SocketAddr::new(IpAddr::V6(ipv6), port))
}
