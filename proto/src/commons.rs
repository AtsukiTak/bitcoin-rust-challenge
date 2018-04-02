#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::net::{IpAddr, SocketAddr};

pub trait AsBytes {
    fn length(&self) -> usize;

    fn write_to(&self, buf: &mut Vec<u8>);
}

macro_rules! endian_struct {
    ($strct: ident, $ty: ty, $size: expr, $endian: ident) => {
        pub struct $strct(pub $ty);

        impl AsBytes for $strct {
            fn length(&self) -> usize {
                $size
            }

            fn write_to(&self, buf: &mut Vec<u8>) {
                let bytes = unsafe { *(&(self.0).$endian() as *const _ as *const [u8; $size]) };
                buf.extend_from_slice(&bytes);
            }
        }
    };
}

endian_struct!(lu16, u16, 2, to_le);
endian_struct!(lu32, u32, 4, to_le);
endian_struct!(lu64, u64, 8, to_le);
endian_struct!(li16, i16, 2, to_le);
endian_struct!(li32, i32, 4, to_le);
endian_struct!(li64, i64, 8, to_le);

endian_struct!(bu16, u16, 2, to_be);
endian_struct!(bu32, u32, 4, to_be);
endian_struct!(bu64, u64, 8, to_be);
endian_struct!(bi16, i16, 2, to_be);
endian_struct!(bi32, i32, 4, to_be);
endian_struct!(bi64, i64, 8, to_be);

pub struct NetAddr {
    time: Option<u32>, // Not present in version message.
    services: u64,
    addr: SocketAddr,
}

impl AsBytes for NetAddr {
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
            lu32(time).write_to(buf);
        }

        // Write services field
        lu64(self.services).write_to(buf);

        // Write ipv6 field
        let ipv6 = match self.addr.ip() {
            IpAddr::V4(v4) => v4.to_ipv6_mapped(),
            IpAddr::V6(v6) => v6,
        };
        buf.extend_from_slice(&ipv6.octets());

        // Write port field
        bu16(self.addr.port()).write_to(buf);
    }
}

pub struct CompactSize(pub u64);

impl AsBytes for CompactSize {
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

    fn write_to(&self, buf: &mut Vec<u8>) {
        let n = self.0;
        if n < 0xFD {
            buf.push(n as u8);
        } else if n <= 0xFFFF {
            buf.push(0xFD);
            lu16(n as u16).write_to(buf);
        } else if n <= 0xFFFF_FFFF {
            buf.push(0xFE);
            lu32(n as u32).write_to(buf);
        } else {
            buf.push(0xFF);
            lu64(n).write_to(buf);
        }
    }
}

pub struct VarStr<'a>(pub &'a str);

impl<'a> AsBytes for VarStr<'a> {
    fn length(&self) -> usize {
        CompactSize(self.0.len() as u64).length() + self.0.len()
    }

    fn write_to(&self, buf: &mut Vec<u8>) {
        CompactSize(self.0.len() as u64).write_to(buf);
        buf.extend_from_slice(self.0.as_bytes());
    }
}
