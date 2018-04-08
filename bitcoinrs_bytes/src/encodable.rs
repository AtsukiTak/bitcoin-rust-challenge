#![allow(non_camel_case_types)]
#![allow(dead_code)]

use super::WriteBuf;

use std::borrow::Borrow;

/// Encodable into an array of byte.
pub trait Encodable {
    /// Length of encoded bytes.
    fn length(&self) -> usize;

    /// Encode `Self` into bytes and write it to buffer.
    /// Note that you don't need to call `WriteBuf::reserve` function
    /// because `WriteBuf::encode` automatically reserves buffer
    /// according to `Encodable::length` function.
    fn encode<W: WriteBuf>(&self, buf: &mut W);

    /// Chain two `Encodable` struct into single.
    /// Since heap allocation is occured once per `WriteBuf::write` call,
    /// chained struct reduce heap allocation.
    fn chain<E>(self, e2: E) -> Chain<Self, E>
    where
        Self: Sized,
        E: Encodable,
    {
        Chain::new(self, e2)
    }
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
    /// It enables you to override `Encodable::encode` function.
    fn encode<W: WriteBuf>(&self, buf: &mut W) {
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

    fn encode<W: WriteBuf>(&self, buf: &mut W) {
        <Self as EncodableSized>::encode(self, buf)
    }
}

pub struct Chain<E1, E2> {
    e1: E1,
    e2: E2,
}

impl<E1, E2> Chain<E1, E2> {
    fn new(e1: E1, e2: E2) -> Chain<E1, E2> {
        Chain { e1: e1, e2: e2 }
    }
}

impl<E1, E2> Encodable for Chain<E1, E2>
where
    E1: Encodable,
    E2: Encodable,
{
    fn length(&self) -> usize {
        self.e1.length() + self.e2.length()
    }

    fn encode<W: WriteBuf>(&self, buf: &mut W) {
        self.e1.encode(buf);
        self.e2.encode(buf);
    }
}
