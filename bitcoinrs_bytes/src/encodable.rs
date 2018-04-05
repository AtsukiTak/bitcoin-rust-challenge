#![allow(non_camel_case_types)]
#![allow(dead_code)]

use super::Bytes;

use std::borrow::Borrow;

/// Encodable into an array of byte.
pub trait Encodable {
    /// Length of encoded bytes.
    fn length(&self) -> usize;

    /// Encode `Self` into bytes and write it to buffer.
    fn encode_to(&self, buf: &mut Bytes);
}

/// Encodable into a sized array of bytes.
pub trait EncodableSized {
    /// Length of encod bytes.
    const SIZE: usize;

    /// Array its size is `Self::SIZE`. In other words, `[u8; Self::SIZE]`.
    type Array: Borrow<[u8]>;

    /// Return encoded bytes.
    fn bytes(&self) -> Self::Array;

    /// Encode `self` into bytes and write it to buffer.
    /// This function is implemented automatically using `bytes` function.
    fn encode_to(&self, buf: &mut Bytes) {
        buf.write_bytes(self.bytes().borrow());
    }
}

impl<T> Encodable for T
where
    T: EncodableSized,
{
    fn length(&self) -> usize {
        T::SIZE
    }

    fn encode_to(&self, buf: &mut Bytes) {
        <Self as EncodableSized>::encode_to(self, buf)
    }
}
