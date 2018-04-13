pub mod version;
pub mod verack;

pub use self::version::VersionMsgPayload;
pub use self::verack::VerackMsgPayload;

use std::io::Cursor;

use bitcoinrs_bytes::{Decodable, Encodable, ReadBuf, ReadError, WriteBuf, endian::u32_l};
use bitcoinrs_crypto::sha256;

use NetworkType;

pub type VersionMsg = Msg<VersionMsgPayload>;
pub type VerackMsg = Msg<VerackMsgPayload>;

#[derive(Debug, Clone)]
pub struct Msg<P: MsgPayload> {
    net_type: NetworkType,
    payload: P,
}

impl<P: MsgPayload> Msg<P> {
    pub fn new(net_type: NetworkType, payload: P) -> Msg<P> {
        Msg {
            net_type: net_type,
            payload: payload,
        }
    }

    pub fn net_type(&self) -> NetworkType {
        self.net_type
    }

    pub fn payload(&self) -> &P {
        &self.payload
    }

    pub fn payload_mut(&mut self) -> &mut P {
        &mut self.payload
    }
}

impl<P: MsgPayload> Encodable for Msg<P> {
    fn length(&self) -> usize {
        24 + self.payload.length()
    }

    fn encode<W: WriteBuf>(&self, buf: &mut W) {
        // Write magic valud.
        buf.write(u32_l::new(self.net_type.magic_num()));

        // Write NULL padded command string
        buf.write_bytes(&P::COMMAND_BYTES);

        // Write length of payload in bytes
        buf.write(u32_l::new(self.payload.length() as u32));

        // Encode payload into bytes.
        let payload = self.payload.to_vec();

        // Compute and write checksum
        let hash = sha256(&sha256(payload.as_slice()));
        buf.write_bytes(&hash[0..4]);

        // Write payload
        buf.write_bytes(&payload.as_slice());
    }
}

impl<P: MsgPayload> Decodable for Msg<P> {
    fn decode<R>(bytes: &mut R) -> Result<Self, ReadError>
    where
        R: ReadBuf,
    {
        // decode network type
        let magic_num = bytes.read::<u32_l>()?.value();
        let net_type = NetworkType::from_magic_num(magic_num).ok_or(ReadError::InvalidBytes)?;

        // read and check command bytes
        {
            let mut command = [0; 12];
            bytes.read_bytes(&mut command)?;
            if command != P::COMMAND_BYTES {
                return Err(ReadError::InvalidBytes);
            }
        };

        // decode length of payload in bytes
        let len = bytes.read::<u32_l>()?.value();

        // decode checksum
        let checksum = {
            let mut buf = [0; 4];
            bytes.read_bytes(&mut buf)?;
            buf
        };

        // read payload bytes
        let payload_bytes = {
            let mut vec = Vec::with_capacity(len as usize);
            vec.write_zeros(len as usize);
            bytes.read_bytes(vec.as_mut_slice())?;
            vec
        };

        // check checksum
        let computed_hash = sha256(&sha256(payload_bytes.as_slice()));
        if &computed_hash[0..4] != &checksum {
            return Err(ReadError::InvalidBytes);
        }

        // decode payload
        let payload = P::decode(&mut Cursor::new(payload_bytes.as_slice()))?;

        Ok(Msg::new(net_type, payload))
    }
}

pub trait MsgPayload: 'static + Sized + Encodable + Decodable {
    const COMMAND_BYTES: [u8; 12];

    fn into_msg(self, net_type: NetworkType) -> Msg<Self> {
        Msg::new(net_type, self)
    }
}
