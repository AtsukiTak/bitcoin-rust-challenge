use bitcoinrs_bytes::{Bytes, BytesMut, Decodable, DecodeError, Encodable};

use msg::MsgPayload;

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
    fn encode(&self, _buf: &mut BytesMut) {}
}

impl Decodable for VerackMsgPayload {
    fn decode(_bytes: &mut Bytes) -> Result<Self, DecodeError> {
        Ok(VerackMsgPayload::new())
    }
}

impl MsgPayload for VerackMsgPayload {
    const COMMAND_BYTES: [u8; 12] = [0x76, 0x65, 0x72, 0x61, 0x63, 0x6b, 0, 0, 0, 0, 0, 0];
}
