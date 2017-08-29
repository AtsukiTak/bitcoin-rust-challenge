use bytes::Bytes;

use error::*;


/// It means "addr"
pub const COMMAND_NAME: [u8; 12] = [b'a', b'd', b'd', b'r', 0, 0, 0, 0, 0, 0, 0, 0];


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddrPayload {
    hoge: usize,
}


pub fn decode(payload: Bytes) -> Result<AddrPayload> {
    panic!()
}


pub fn encode(addr: AddrPayload) -> Result<Bytes> {
    panic!()
}
