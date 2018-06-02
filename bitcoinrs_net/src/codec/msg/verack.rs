use bitcoinrs_bytes::encode::{Encodable, WriteBuffer};
use bitcoinrs_bytes::decode::{Decodable, DecodeError, ReadBuffer};

use super::MsgPayload;

#[derive(Debug, Clone, Copy)]
pub struct VerackMsgPayload(());

impl VerackMsgPayload {
    pub fn new() -> VerackMsgPayload {
        VerackMsgPayload(())
    }
}

impl Encodable for VerackMsgPayload {
    fn length(&self) -> usize {
        0
    }

    /// Nothing to encode.
    fn encode<W: WriteBuffer>(&self, _buf: &mut W) {}
}

impl Decodable for VerackMsgPayload {
    fn decode<R: ReadBuffer>(_bytes: &mut R) -> Result<Self, DecodeError> {
        Ok(VerackMsgPayload::new())
    }
}

impl MsgPayload for VerackMsgPayload {
    const COMMAND_BYTES: [u8; 12] = [0x76, 0x65, 0x72, 0x61, 0x63, 0x6b, 0, 0, 0, 0, 0, 0];
}
