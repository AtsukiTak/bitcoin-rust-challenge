#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::borrow::Borrow;

/// Encodable into an array of byte.
pub trait Encodable {
    /// Length of encoded bytes.
    fn length(&self) -> usize;

    /// Encode `Self` into bytes and write it to buffer.
    /// Note that this operation **MUST** be atomic; write whole bytes or nothing.
    /// `Encodable::chain` helps to handle encode multi `Encodable` items.
    /// And also, `WriteBuffer::has_buffer` function is useful as well.
    ///
    /// # Panic
    /// when underlying buffer has not enough buffer to write.
    fn encode<W: WriteBuffer>(&self, buf: &mut W);

    /// Chain two `Encodable` struct into single.
    /// This operation is atomic.
    fn chain<'a, 'b, E>(&'a self, e2: &'b E) -> Chain<'a, 'b, Self, E>
    where
        Self: Sized,
        E: Encodable,
    {
        Chain::new(self, e2)
    }

    /// Convenient function to create Vec<u8> representing encoded bytes.
    fn to_vec(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(self.length());
        self.encode(&mut vec);
        vec
    }
}

impl<'a> Encodable for &'a [u8] {
    fn length(&self) -> usize {
        self.len()
    }

    fn encode<W: WriteBuffer>(&self, buf: &mut W) {
        buf.write_bytes(self)
    }
}

pub trait WriteBuffer: Sized {
    /// Write an array of bytes into buffer.
    /// This operation is atomic; write all bytes or nothing.
    ///
    /// # Panic
    /// when underlying buffer has not enough buffer to write.
    fn write_bytes(&mut self, bytes: &[u8]);

    /// Check whether this buffer has enough buffer.
    fn has_buffer(&self, size: usize) -> bool;

    fn write<E: Encodable>(&mut self, e: E) {
        e.encode(self)
    }
}

impl WriteBuffer for Vec<u8> {
    fn write_bytes(&mut self, bytes: &[u8]) {
        self.extend_from_slice(bytes);
    }

    fn has_buffer(&self, _size: usize) -> bool {
        true
    }
}

impl<'a> WriteBuffer for ::std::io::Cursor<&'a mut [u8]> {
    fn write_bytes(&mut self, bytes: &[u8]) {
        let bytes_len = bytes.len();
        let start_pos = self.position() as usize; // Should I check here?

        self.set_position((start_pos + bytes_len) as u64);

        let buf = &mut self.get_mut()[start_pos..];

        (&mut buf[..bytes_len]).copy_from_slice(bytes);
    }

    fn has_buffer(&self, size: usize) -> bool {
        let current_pos = self.position() as usize;
        if current_pos + size <= self.get_ref().len() {
            true
        } else {
            false
        }
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
    fn encode_with_bytes<W: WriteBuffer>(&self, buf: &mut W) {
        buf.write_bytes(self.bytes().borrow())
    }
}

impl<T> Encodable for T
where
    T: EncodableSized,
{
    fn length(&self) -> usize {
        T::SIZE
    }

    fn encode<W: WriteBuffer>(&self, buf: &mut W) {
        self.encode_with_bytes(buf)
    }
}

pub struct Chain<'a, 'b, E1: 'a, E2: 'b> {
    e1: &'a E1,
    e2: &'b E2,
}

impl<'a, 'b, E1, E2> Chain<'a, 'b, E1, E2> {
    fn new(e1: &'a E1, e2: &'b E2) -> Chain<'a, 'b, E1, E2> {
        Chain { e1: e1, e2: e2 }
    }
}

impl<'a, 'b, E1, E2> Encodable for Chain<'a, 'b, E1, E2>
where
    E1: Encodable,
    E2: Encodable,
{
    fn length(&self) -> usize {
        self.e1.length() + self.e2.length()
    }

    fn encode<W: WriteBuffer>(&self, buf: &mut W) {
        self.e1.encode(buf);
        self.e2.encode(buf);
    }
}
