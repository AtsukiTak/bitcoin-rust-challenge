use {Decodable, Encodable};

pub trait WriteBuf {
    /// Reserve buffer at least `additional` size.
    /// When heap allocation is needed, this function must be called.
    fn reserve(&mut self, additional: usize);

    /// Write some bytes into buffer.
    /// Note that if buffer size is shorter than bytes size,
    /// `reserve` is called.
    fn write_bytes(&mut self, bytes: &[u8]);

    /// Write some zeros into buffer.
    /// Note that if buffer size is shorter than `size`,
    /// `reserve` is called.
    fn write_zeros(&mut self, size: usize);

    /// Write an encodable struct into buffer.
    fn write<E>(&mut self, e: E)
    where
        Self: Sized,
        E: Encodable,
    {
        self.reserve(e.length());
        e.encode(self);
    }
}

impl WriteBuf for Vec<u8> {
    fn reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        let old_len = self.len();
        self.write_zeros(bytes.len());
        self.as_mut_slice()[old_len..].copy_from_slice(bytes);
    }

    fn write_zeros(&mut self, size: usize) {
        self.reserve(size);
        unsafe {
            let old_len = self.len();
            self.set_len(old_len + size);
        }
    }
}

pub trait ReadBuf {
    /// Read some bytes and advance cursor.
    /// Note that even when Err is returned, buf may be filled by some bytes.
    fn read_bytes(&mut self, buf: &mut [u8]) -> Result<(), ReadError>;

    /// Read a struct which implement `Decodable`.
    fn read<D>(&mut self) -> Result<D, ReadError>
    where
        Self: Sized,
        D: Decodable,
    {
        D::decode(self)
    }
}

use std::io::{Cursor, Read};

impl<'a> ReadBuf for Cursor<&'a [u8]> {
    fn read_bytes(&mut self, buf: &mut [u8]) -> Result<(), ReadError> {
        self.read_exact(buf).map_err(|_| ReadError::ShortLength)
    }
}

pub enum ReadError {
    ShortLength,
    InvalidBytes,
    UnderlyingError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_zeros() {
        let mut vec = Vec::new();
        vec.write_zeros(100);
        assert_eq!(vec.as_slice(), &[0u8; 100][..]);
    }

    #[test]
    fn write_bytes() {
        let mut vec = Vec::new();
        let bytes = [1, 2, 3, 4, 5, 6];
        vec.write_bytes(&bytes[0..3]);
        vec.write_bytes(&bytes[3..5]);
        assert_eq!(vec.as_slice(), &bytes[0..5])
    }

    #[test]
    fn read_bytes() {
        let bytes: [u8; 6] = [0, 1, 2, 3, 4, 5];
        let mut buf = [0; 3];
        let mut buf2 = [0; 3];
        let mut read = Cursor::new(&bytes[..]);
        read.read_bytes(&mut buf);
        read.read_bytes(&mut buf2);
        assert_eq!(buf, [0, 1, 2]);
        assert_eq!(buf2, [3, 4, 5]);
    }
}
