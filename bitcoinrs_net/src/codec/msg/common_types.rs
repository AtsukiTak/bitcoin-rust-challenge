#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::net::Ipv6Addr;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::Cursor;

use bitcoinrs_bytes::decode::{Decodable, DecodeError, ReadBuffer};
use bitcoinrs_bytes::encode::{Encodable, EncodableSized, WriteBuffer};
use bitcoinrs_bytes::endian::{u16_b, u16_l, u32_l, u64_l};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    fn decode<R: ReadBuffer>(bytes: &mut R) -> Result<Timestamp, DecodeError> {
        Ok(Timestamp(bytes.read::<u64_l>()?.value()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

    fn encode<W: WriteBuffer>(&self, buf: &mut W) {
        if self.0 < 0xFD {
            buf.write(self.0 as u8);
        } else if self.0 <= 0xFFFF {
            buf.write(0xFD_u8);
            buf.write(u16_l::new(self.0 as u16));
        } else if self.0 <= 0xFFFF_FFFF {
            buf.write(0xFE_u8);
            buf.write(u32_l::new(self.0 as u32));
        } else {
            buf.write(0xFF_u8);
            buf.write(u64_l::new(self.0));
        }
    }
}

impl Decodable for CompactSize {
    fn decode<R: ReadBuffer>(bytes: &mut R) -> Result<CompactSize, DecodeError> {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VarStr(pub String);

impl<'a> Encodable for &'a VarStr {
    fn length(&self) -> usize {
        CompactSize(self.0.len() as u64).length() + self.0.len()
    }

    fn encode<W: WriteBuffer>(&self, buf: &mut W) {
        buf.write(CompactSize(self.0.len() as u64).chain(&self.0.as_bytes()))
    }
}

impl Decodable for VarStr {
    fn decode<R: ReadBuffer>(buf: &mut R) -> Result<VarStr, DecodeError> {
        let len = buf.read::<CompactSize>()?.0;
        let s = String::from_utf8(buf.read_bytes(len as usize)?.to_vec())
            .map_err(|_| DecodeError::InvalidBytes)?;
        Ok(VarStr(s))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Service {
    Network = 1,
    Getutxo = 2,
    Bloom = 4,
    Witness = 8,
    NetworkLimited = 1024,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    fn decode<R: ReadBuffer>(bytes: &mut R) -> Result<Services, DecodeError> {
        Ok(Services(bytes.read::<u64_l>()?.value()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NetAddr {
    ts: Timestamp, // Not present in version message.
    services: Services,
    ip: Ipv6Addr,
    port: u16,
}

impl NetAddr {
    pub fn new(ts: Timestamp, services: Services, ip: Ipv6Addr, port: u16) -> NetAddr {
        NetAddr {
            ts: ts,
            services: services,
            ip: ip,
            port: port,
        }
    }
}

impl EncodableSized for NetAddr {
    const SIZE: usize = 30;
    type Array = [u8; 30];

    #[allow(unused_must_use)]
    fn bytes(&self) -> [u8; 30] {
        let mut bytes = [0; 30];
        {
            let mut buf = Cursor::new(&mut bytes[..]);

            // never error
            buf.write(self.ts);
            buf.write(self.services);
            buf.write(&self.ip.octets()[..]);
            buf.write(u16_b::new(self.port));
        }
        bytes
    }
}

impl Decodable for NetAddr {
    fn decode<R: ReadBuffer>(buf: &mut R) -> Result<NetAddr, DecodeError> {
        let ts = buf.read::<Timestamp>()?;
        let services = buf.read::<Services>()?;
        let ipv6 = Ipv6Addr::from(buf.read::<[u8; 16]>()?);
        let port = buf.read::<u16_l>()?.value();
        Ok(NetAddr {
            ts: ts,
            services: services,
            ip: ipv6,
            port: port,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NetAddrForVersionMsg {
    pub services: Services,
    pub ip: Ipv6Addr,
    pub port: u16,
}

impl NetAddrForVersionMsg {
    pub fn new(services: Services, ip: Ipv6Addr, port: u16) -> NetAddrForVersionMsg {
        NetAddrForVersionMsg {
            services: services,
            ip: ip,
            port: port,
        }
    }
}

impl EncodableSized for NetAddrForVersionMsg {
    const SIZE: usize = 26;
    type Array = [u8; 26];

    #[allow(unused_must_use)]
    fn bytes(&self) -> [u8; 26] {
        let mut bytes = [0; 26];
        {
            let mut buf = Cursor::new(&mut bytes[..]);
            buf.write(self.services);
            buf.write(&self.ip.octets()[..]);
            buf.write(u16_b::new(self.port));
        }
        bytes
    }
}

impl Decodable for NetAddrForVersionMsg {
    fn decode<R: ReadBuffer>(buf: &mut R) -> Result<NetAddrForVersionMsg, DecodeError> {
        let services = buf.read::<Services>()?;
        let ipv6 = Ipv6Addr::from(buf.read::<[u8; 16]>()?);
        let port = buf.read::<u16_l>()?.value();
        Ok(NetAddrForVersionMsg {
            services: services,
            ip: ipv6,
            port: port,
        })
    }
}
