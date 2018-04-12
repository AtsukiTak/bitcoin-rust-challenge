use bitcoinrs_bytes::{Decodable, Encodable, ReadBuf, ReadError, WriteBuf};

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
    {
        Ok(VerackMsgPayload::new())
    }
}
