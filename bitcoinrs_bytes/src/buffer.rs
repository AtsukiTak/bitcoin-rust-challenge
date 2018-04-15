use encode::{EncodeError, WriteBuffer};

pub struct Buffer {
    bytes: Vec<u8>,
    discarded: usize,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            bytes: Vec::new(),
            discarded: 0,
        }
    }

    pub fn drop_front(&mut self, size: usize) {
        assert!(self.discarded + size <= self.bytes.len());

        self.discarded += size;

        const DROP_THRESHOULD: usize = 128;
        if DROP_THRESHOULD < self.discarded {
            let new_head_ptr = self.bytes.as_mut_ptr();
            let current_head_ptr = self.as_ref().as_ptr();
            let new_len = self.bytes.len() - self.discarded;
            unsafe {
                ::std::intrinsics::copy(current_head_ptr, new_head_ptr, new_len);
                self.bytes.set_len(new_len);
            }
        }
    }
}

impl WriteBuffer for Buffer {
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        self.bytes.extend_from_slice(bytes);
        Ok(())
    }
}

impl AsRef<[u8]> for Buffer {
    fn as_ref(&self) -> &[u8] {
        &self.bytes.as_slice()[self.discarded..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drop_front() {
        let mut buffer = Buffer::new();
        let bytes = [1, 2, 3, 4, 5];
        let zeros_64 = [0; 64];
        buffer.write_bytes(&bytes[..]).unwrap();
        assert!(&buffer.as_ref()[0..5] == &bytes[0..5]);

        buffer.drop_front(3);
        assert!(&buffer.as_ref()[0..2] == &bytes[3..5]);

        buffer.write_bytes(&zeros_64[..]).unwrap();
        buffer.write_bytes(&bytes[..]).unwrap();
        buffer.drop_front(64);
        assert_eq!(&buffer.as_ref()[0..5], &[0, 0, 1, 2, 3][..]);
    }
}
