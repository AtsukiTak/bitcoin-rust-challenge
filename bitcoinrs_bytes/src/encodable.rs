#![allow(non_camel_case_types)]
#![allow(dead_code)]

use BytesMut;

use std::borrow::Borrow;

/// Encodable into an array of byte.
pub trait Encodable {
    /// Length of encoded bytes.
    fn length(&self) -> usize;

    /// Encode `Self` into bytes and write it to buffer.
    fn encode(&self, buf: &mut BytesMut);

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

    /// Convenient function to create Vec<u8> representing encoded bytes.
    fn to_vec(&self) -> Vec<u8> {
        let mut bytes = BytesMut::new();
        bytes.reserve(self.length());
        self.encode(&mut bytes);
        bytes.to_vec()
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
    fn encode_with_bytes(&self, buf: &mut BytesMut) {
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

    fn encode(&self, buf: &mut BytesMut) {
        self.encode_with_bytes(buf)
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

    fn encode(&self, buf: &mut BytesMut) {
        self.e1.encode(buf);
        self.e2.encode(buf);
    }
}
