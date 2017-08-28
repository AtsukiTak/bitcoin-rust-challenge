use bytes::Bytes;

use error::*;


/// It means "addr"
pub const COMMAND_NAME: [u8; 12] = [0x61, 0x64, 0x64, 0x72, 0, 0, 0, 0, 0, 0, 0, 0];


pub struct AddrPayload {
    hoge: usize,
}


pub fn decode(payload: Bytes) -> Result<AddrPayload> {
    panic!()
}


pub fn command_name_and_payload(addr: AddrPayload) -> Result<([u8; 12], Bytes)> {
    panic!()
}
