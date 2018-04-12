pub mod version;
pub mod verack;

pub use self::version::VersionMsgPayload;
pub use self::verack::VerackMsgPayload;

use std::io::Cursor;

use bitcoinrs_bytes::{Decodable, Encodable, ReadBuf, ReadError, WriteBuf, endian::u32_l};
use bitcoinrs_crypto::sha256;

use NetworkType;

#[derive(Debug, Clone)]
pub struct Msg {
    net_type: NetworkType,
    payload: MsgPayload,
}

#[derive(Debug, Clone)]
pub enum MsgPayload {
    Version(VersionMsgPayload),
    Verack,
}

impl Msg {
    pub fn new(net_type: NetworkType, payload: MsgPayload) -> Msg {
        Msg {
            net_type: net_type,
            payload: payload,
        }
    }

    pub fn net_type(&self) -> NetworkType {
        self.net_type
    }

    pub fn payload(&self) -> &MsgPayload {
        &self.payload
    }
}

impl Encodable for Msg {
    fn length(&self) -> usize {
        24 + self.payload.length()
    }

    fn encode<W: WriteBuf>(&self, buf: &mut W) {
        // Write magic valud.
        buf.write(u32_l::new(self.net_type.magic_num()));

        // Write NULL padded command string
        buf.write_bytes(&self.payload.command_bytes());

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

impl Decodable for Msg {
    fn decode<R>(bytes: &mut R) -> Result<Self, ReadError>
    where
        R: ReadBuf,
    {
        // decode network type
        let magic_num = bytes.read::<u32_l>()?.value();
        let net_type = NetworkType::from_magic_num(magic_num).ok_or(ReadError::InvalidBytes)?;

        // read command bytes
        let command_bytes = {
            let mut command = [0; 12];
            bytes.read_bytes(&mut command)?;
            command
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
        let payload = MsgPayload::decode_with_command_bytes(
            command_bytes,
            &mut Cursor::new(payload_bytes.as_slice()),
        )?;

        Ok(Msg::new(net_type, payload))
    }
}

const COMMAND_BYTES_VERSION: [u8; 12] = [0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0, 0, 0, 0, 0];
const COMMAND_BYTES_VERACK: [u8; 12] = [0x76, 0x65, 0x72, 0x61, 0x63, 0x6b, 0, 0, 0, 0, 0, 0];

impl MsgPayload {
    pub fn command_bytes(&self) -> [u8; 12] {
        match self {
            &MsgPayload::Version(_) => COMMAND_BYTES_VERSION,
            &MsgPayload::Verack => COMMAND_BYTES_VERACK,
        }
    }

    pub fn is_version(&self) -> Option<&VersionMsgPayload> {
        match self {
            &MsgPayload::Version(ref p) => Some(p),
            _ => None,
        }
    }

    pub fn is_verack(&self) -> Option<()> {
        match self {
            &MsgPayload::Verack => Some(()),
            _ => None,
        }
    }

    pub fn decode_with_command_bytes<R: ReadBuf>(
        command: [u8; 12],
        read_buf: &mut R,
    ) -> Result<MsgPayload, ReadError> {
        match command {
            COMMAND_BYTES_VERSION => Ok(MsgPayload::Version(read_buf.read()?)),
            COMMAND_BYTES_VERACK => Ok(MsgPayload::Verack),
            _ => Err(ReadError::InvalidBytes),
        }
    }
}

impl Encodable for MsgPayload {
    fn length(&self) -> usize {
        match self {
            &MsgPayload::Version(ref p) => p.length(),
            &MsgPayload::Verack => 0,
        }
    }

    fn encode<W: WriteBuf>(&self, buf: &mut W) {
        match self {
            &MsgPayload::Version(ref p) => p.encode(buf),
            &MsgPayload::Verack => (),
        }
    }
}
