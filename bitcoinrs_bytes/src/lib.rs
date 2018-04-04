#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::borrow::Borrow;

pub trait Bytes {
    fn length(&self) -> usize;

    fn write_to(&self, buf: &mut Vec<u8>);
}

pub trait SizedBytes {
    const SIZE: usize;

    type Array: Borrow<[u8]>;

    fn bytes(&self) -> Self::Array;
}

impl<T> Bytes for T
where
    T: SizedBytes,
{
    fn length(&self) -> usize {
        T::SIZE
    }

    fn write_to(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(self.bytes().borrow());
    }
}

macro_rules! impl_sized_bytes {
    ($t: ty, $size: expr) => {
        impl SizedBytes for $t {
            const SIZE: usize = $size;

            type Array = [u8; $size];

            fn bytes(&self) -> [u8; $size] {
                unsafe { *(&self as *const _ as *const [u8; $size]) }
            }
        }
    };
}

impl_sized_bytes!(u8, 1);
impl_sized_bytes!(u16, 2);
impl_sized_bytes!(u32, 4);
impl_sized_bytes!(u64, 8);
impl_sized_bytes!(i8, 1);
impl_sized_bytes!(i16, 2);
impl_sized_bytes!(i32, 4);
impl_sized_bytes!(i64, 8);

pub struct CompactSize(pub u64);

impl Bytes for CompactSize {
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
            n.to_le().write_to(buf);
        } else if n <= 0xFFFF_FFFF {
            buf.push(0xFE);
            n.to_le().write_to(buf);
        } else {
            buf.push(0xFF);
            n.to_le().write_to(buf);
        }
    }
}

pub struct VarStr<'a>(pub &'a str);

impl<'a> Bytes for VarStr<'a> {
    fn length(&self) -> usize {
        CompactSize(self.0.len() as u64).length() + self.0.len()
    }

    fn write_to(&self, buf: &mut Vec<u8>) {
        CompactSize(self.0.len() as u64).write_to(buf);
        buf.extend_from_slice(self.0.as_bytes());
    }
}
