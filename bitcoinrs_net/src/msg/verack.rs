use bitcoinrs_bytes::{Decodable, Encodable, WriteBuf, ReadBuf, ReadError};

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
    fn encode<W: WriteBuf>(&self, _buf: &mut W) {}
}

impl Decodable for VerackMsgPayload {
    fn decode<R>(_bytes: &mut R) -> Result<Self, ReadError>
    where
        R: ReadBuf,
        Self: Sized,
    {
        Ok(VerackMsgPayload::new())
    }
}

impl MsgPayload for VerackMsgPayload {
    const COMMAND: &'static str = "verack";
}
