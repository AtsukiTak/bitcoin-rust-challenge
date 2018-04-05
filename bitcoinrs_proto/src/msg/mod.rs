pub mod version;

pub use self::version::VersionMsg;

use bitcoinrs_bytes::Bytes;
use bitcoinrs_crypto::sha256;

use NetworkType;

pub struct Msg<M: MsgPayload> {
    magic: u32, // little endian
    payload: M,
}

impl<M: MsgPayload> Bytes for Msg<M> {
    fn length(&self) -> usize {
        24 + self.payload.length()
    }

    fn write_to(&self, buf: &mut Vec<u8>) {
        // Write magic valud.
        self.magic.write_to(buf);

        // Write NULL padded command string
        write_command(M::COMMAND, buf);

        // Write length of payload in bytes
        (self.payload.length() as u32).to_le().write_to(buf);

        // Write temporary checksum
        buf.extend_from_slice(&[0; 4][..]);

        // Write payload
        self.payload.write_to(buf);

        // Compute and write checksum
        const START_PAYLOAD: usize = 24;
        const START_CHECKSUM: usize = 20;
        let hash = sha256(&sha256(&buf.as_slice()[START_PAYLOAD..]));
        buf.as_mut_slice()[START_CHECKSUM..START_CHECKSUM + 4].copy_from_slice(&hash[0..4]);
    }
}

impl<M: MsgPayload> Msg<M> {
    pub fn new(network: NetworkType, payload: M) -> Msg<M> {
        Msg {
            magic: network.magic_num().to_le(),
            payload: payload,
        }
    }
}

/// Marker trait for Bitcoin p2p message payload.
pub trait MsgPayload: Bytes {
    const COMMAND: &'static str;
}

fn write_command(command: &str, buf: &mut Vec<u8>) {
    assert!(command.len() <= 11);

    let mut bytes: [u8; 12] = [0; 12];
    bytes.copy_from_slice(command.as_bytes());
    buf.extend_from_slice(&bytes);
}
