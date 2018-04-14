use {Decodable, DecodeError, Encodable};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bytes<'a>(&'a [u8]);

impl<'a> Bytes<'a> {
    pub fn new(bytes: &'a [u8]) -> Bytes<'a> {
        Bytes(bytes)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Read some bytes from buffer without consuming it.
    /// # Panic
    /// when length of `buf` is larger than buffered length.
    pub fn peek(&self, buf: &mut [u8]) {
        let buf_len = buf.len();
        assert!(buf_len <= self.len());

        buf.copy_from_slice(&self.0[..buf_len]);
    }

    /// Consume specified size of bytes.
    /// # Panic
    /// when `size` is larger than bufferd length.
    pub fn consume(&mut self, size: usize) {
        self.0 = &self.0[size..];
    }

    /// Read and consume some bytes.
    pub fn read_bytes(&mut self, buf: &mut [u8]) -> Result<(), DecodeError>{
        if self.len() < buf.len() {
            return Err(DecodeError::ShortLength);
        }

        self.peek(buf);
        self.consume(buf.len());
        Ok(())
    }

    pub fn read<D: Decodable>(&mut self) -> Result<D, DecodeError> {
        D::decode(self)
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0
    }
}

impl<'a> Encodable for Bytes<'a> {
    fn length(&self) -> usize {
        self.len()
    }

    fn encode(&self, buf: &mut BytesMut) {
        buf.write_bytes(self.as_slice())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BytesMut(Vec<u8>);

impl BytesMut {
    pub fn new() -> BytesMut {
        BytesMut(Vec::new())
    }

    pub fn with_vec(vec: Vec<u8>) -> BytesMut {
        BytesMut(vec)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn bytes(&self) -> Bytes {
        Bytes::new(self.0.as_slice())
    }

    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        let old_len = self.len();
        self.write_zeros(bytes.len());
        self.0.as_mut_slice()[old_len..].copy_from_slice(bytes);
    }

    pub fn write_zeros(&mut self, size: usize) {
        unsafe {
            let old_len = self.len();
            self.0.set_len(old_len + size);
        }
    }

    pub fn write<E: Encodable>(&mut self, e: E) {
        e.encode(self)
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.0.as_mut_slice()
    }

    /// Zero cost conversion into Vec<u8>.
    pub fn to_vec(self) -> Vec<u8> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_zeros() {
        let mut bytes = BytesMut::new();
        bytes.write_zeros(100);
        assert_eq!(bytes.as_slice(), &[0u8; 100][..]);
    }

    #[test]
    fn write_bytes() {
        let mut vec = BytesMut::new();
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
        let mut read = Bytes::new(&bytes[..]);
        read.read_bytes(&mut buf);
        read.read_bytes(&mut buf2);
        assert_eq!(buf, [0, 1, 2]);
        assert_eq!(buf2, [3, 4, 5]);
    }
}
