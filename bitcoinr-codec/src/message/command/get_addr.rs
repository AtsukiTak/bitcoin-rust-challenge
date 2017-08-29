use bytes::Bytes;

use error::*;

/// It means "getaddr"
pub const COMMAND_NAME: [u8; 12] = [0x67, 0x65, 0x74, 0x61, 0x64, 0x64, 0x72, 0, 0, 0, 0, 0];


pub fn encode() -> Result<Bytes> {
    Ok(Bytes::new())
}
