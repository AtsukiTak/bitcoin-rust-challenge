pub mod version;

pub use self::version::VersionMsgPayload;

use bitcoinrs_bytes::{Encodable, WriteBuf, endian::u32_l};
use bitcoinrs_crypto::sha256;

use NetworkType;

#[derive(Debug, Clone, Copy)]
pub struct Msg<M: MsgPayload> {
    magic: u32,
    payload: M,
}

impl<M: MsgPayload> Msg<M> {
    pub fn new(network: NetworkType, payload: M) -> Msg<M> {
        Msg {
            magic: network.magic_num(),
            payload: payload,
        }
    }
}

impl<M: MsgPayload> Encodable for Msg<M> {
    fn length(&self) -> usize {
        24 + self.payload.length()
    }

    fn encode<W: WriteBuf>(&self, buf: &mut W) {
        // Write magic valud.
        buf.write(u32_l::new(self.magic));

        // Write NULL padded command string
        write_command(M::COMMAND, buf);

        // Write length of payload in bytes
        buf.write(u32_l::new(self.payload.length() as u32));

        let payload = self.payload.to_vec();

        // Compute and write checksum
        let hash = sha256(&sha256(payload.as_slice()));
        buf.write_bytes(&hash[0..4]);

        // Write payload
        buf.write_bytes(&payload.as_slice());
    }
}

/// Marker trait for Bitcoin p2p message payload.
pub trait MsgPayload: Encodable {
    const COMMAND: &'static str;

    fn into_msg(self, network: NetworkType) -> Msg<Self>
    where
        Self: Sized,
    {
        Msg::new(network, self)
    }
}

fn write_command<W: WriteBuf>(command: &str, buf: &mut W) {
    assert!(command.len() <= 11);

    let mut bytes: [u8; 12] = [0; 12];
    (&mut bytes[..command.len()]).copy_from_slice(command.as_bytes());
    buf.write_bytes(&bytes);
}
