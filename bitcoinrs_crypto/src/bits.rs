pub struct Bits {
    bytes: Vec<u8>,
    next_bit_shift: usize,
}

impl Bits {
    pub fn new() -> Bits {
        Bits {
            bytes: Vec::new(),
            next_bit_shift: 7,
        }
    }

    pub fn push(&mut self, bit: bool) {
        let bit_as_byte = (bit as u8) << self.next_bit_shift;

        if self.next_bit_shift == 7 {
            self.bytes.push(bit_as_byte);
        } else {
            let last_byte = self.bytes.pop().unwrap();
            let new_byte = last_byte | bit_as_byte;
            self.bytes.push(new_byte);
        }
    }

    pub fn push_bytes(&mut self, byte: u8) {
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bits() {
        let mut bits = Bits::new();
        bits.push(true);
    }
}
