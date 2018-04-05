#![allow(non_camel_case_types)]
#![allow(dead_code)]

mod encodable;
mod endian;

use std::borrow::Borrow;

/// Representing array of byte.
/// For now, it just wrapper of `Vec<u8`.
pub struct Bytes(Vec<u8>);

impl Bytes {
    /// Create a new `Bytes`.
    pub fn new() -> Bytes {
        Bytes(Vec::new())
    }

    /// Create a new `Bytes` with capacity.
    pub fn with_capacity(cap: usize) -> Bytes {
        Bytes(Vec::with_capacity(cap))
    }

    /// Create a new `Bytes` from `Vec<u8>`.
    pub fn from_vec(vec: Vec<u8>) -> Bytes {
        Bytes(vec)
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn as_mut_slice(&self) -> &mut [u8] {
        self.0.as_mut_slice()
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.0.extend_from_slice(bytes);
    }

    pub fn write_encodable<E: Encodable>(&mut self, e: E) {
        e.encode_to(self)
    }
}
