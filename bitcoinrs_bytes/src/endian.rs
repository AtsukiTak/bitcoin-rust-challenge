pub struct u16_l(u16);
pub struct u32_l(u32);
pub struct u64_l(u64);
pub struct u16_b(u16);
pub struct u32_b(u32);
pub struct u64_b(u64);

use std::fmt::{Debug, Display, Error as FmtError, Formatter};

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

        impl ::EncodableSized for $t {
            const SIZE: usize = $size;

            type Array = [u8; $size];

            fn bytes(&self) -> [u8; $size] {
                unsafe { *(&self.0 as *const _ as *const [u8; $size]) }
            }
        }

        impl ::Decodable for $t {
            fn decode<R: ::ReadBuf>(reader: &mut R) -> Result<$t, ::ReadError> {
                let mut buf: [u8; $size] = [0; $size];
                reader.read_bytes(&mut buf)?;
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

impl_prim_endian!(u16_l, u16_l, u16, 2, "Big endian", to_le, u16::from_le);
impl_prim_endian!(u32_l, u32_l, u32, 4, "Big endian", to_le, u32::from_le);
impl_prim_endian!(u64_l, u64_l, u64, 8, "Big endian", to_le, u64::from_le);
impl_prim_endian!(u16_b, u16_b, u16, 2, "Little endian", to_be, u16::from_be);
impl_prim_endian!(u32_b, u32_b, u32, 4, "Little endian", to_be, u32::from_be);
impl_prim_endian!(u64_b, u64_b, u64, 8, "Little endian", to_be, u64::from_be);
