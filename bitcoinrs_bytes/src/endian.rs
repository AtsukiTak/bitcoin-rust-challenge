#![allow(non_camel_case_types)]

pub struct u16_l(u16);
pub struct u32_l(u32);
pub struct u64_l(u64);
pub struct u16_b(u16);
pub struct u32_b(u32);
pub struct u64_b(u64);
pub struct i16_l(i16);
pub struct i32_l(i32);
pub struct i64_l(i64);
pub struct i16_b(i16);
pub struct i32_b(i32);
pub struct i64_b(i64);

use std::fmt::{Debug, Display, Error as FmtError, Formatter};

use {Bytes, Decodable, EncodableSized, DecodeError};

macro_rules! impl_prim_endian {
    ($t: ty, $t_exp: expr, $inner_t: ty, $size: expr, $en_s: expr, $to_en: ident, $from_en: path) => {
        impl $t {
            /// Create a new $t.
            pub fn new(n: $inner_t) -> $t {
                $t_exp(n.$to_en())
            }

            /// Get inner value whose endian is os specific.
            pub fn value(&self) -> $inner_t {
                $from_en(self.0)
            }
        }

        impl EncodableSized for $t {
            const SIZE: usize = $size;

            type Array = [u8; $size];

            fn bytes(&self) -> [u8; $size] {
                unsafe { ::std::mem::transmute::<$inner_t, [u8; $size]>(self.0) }
            }
        }

        impl Decodable for $t {
            fn decode(bytes: &mut Bytes) -> Result<$t, DecodeError> {
                let mut buf: [u8; $size] = [0; $size];
                bytes.read_bytes(&mut buf)?;
                let raw_num = unsafe { *(&buf as *const _ as *const $inner_t) };
                Ok($t_exp(raw_num))
            }
        }

        impl Debug for $t {
            fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
                // ex) "Big endian 42"
                write!(f, "{} {}", $en_s, self.value())
            }
        }

        impl Display for $t {
            fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
                // ex) "42"
                write!(f, "{}", self.value())
            }
        }
    };
}

impl_prim_endian!(u16_l, u16_l, u16, 2, "Little endian", to_le, u16::from_le);
impl_prim_endian!(u32_l, u32_l, u32, 4, "Little endian", to_le, u32::from_le);
impl_prim_endian!(u64_l, u64_l, u64, 8, "Little endian", to_le, u64::from_le);
impl_prim_endian!(u16_b, u16_b, u16, 2, "Big endian", to_be, u16::from_be);
impl_prim_endian!(u32_b, u32_b, u32, 4, "Big endian", to_be, u32::from_be);
impl_prim_endian!(u64_b, u64_b, u64, 8, "Big endian", to_be, u64::from_be);
impl_prim_endian!(i16_l, i16_l, i16, 2, "Little endian", to_le, i16::from_le);
impl_prim_endian!(i32_l, i32_l, i32, 4, "Little endian", to_le, i32::from_le);
impl_prim_endian!(i64_l, i64_l, i64, 8, "Little endian", to_le, i64::from_le);
impl_prim_endian!(i16_b, i16_b, i16, 2, "Big endian", to_be, i16::from_be);
impl_prim_endian!(i32_b, i32_b, i32, 4, "Big endian", to_be, i32::from_be);
impl_prim_endian!(i64_b, i64_b, i64, 8, "Big endian", to_be, i64::from_be);

impl EncodableSized for u8 {
    const SIZE: usize = 1;
    type Array = [u8; 1];

    fn bytes(&self) -> [u8; 1] {
        [*self]
    }
}

impl Decodable for u8 {
    fn decode(bytes: &mut Bytes) -> Result<u8, DecodeError> {
        let mut buf = [0];
        bytes.read_bytes(&mut buf)?;
        Ok(buf[0])
    }
}

impl EncodableSized for i8 {
    const SIZE: usize = 1;
    type Array = [u8; 1];

    fn bytes(&self) -> [u8; 1] {
        [*self as u8]
    }
}

impl Decodable for i8 {
    fn decode(bytes: &mut Bytes) -> Result<i8, DecodeError> {
        Ok(bytes.read::<u8>()? as i8)
    }
}
