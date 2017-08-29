use bytes::Bytes;

use error::*;

/// It means "getaddr"
pub const COMMAND_NAME: [u8; 12] = [b'g', b'e', b't', b'a', b'd', b'd', b'r', 0, 0, 0, 0, 0];


pub fn encode() -> Result<Bytes> {
    Ok(Bytes::new())
}
