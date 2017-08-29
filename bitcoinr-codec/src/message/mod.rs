mod decoder;
mod encoder;
pub mod command;

pub use self::decoder::decode_message;
pub use self::encoder::encode_message;

use tokio_io::codec::{Decoder, Encoder};

use bytes::BytesMut;

use self::command::{version, addr};
use net::NetworkType;
use error::*;


pub const SIZE_OF_HEADER: usize = 24;

pub const EMPTY_STRING_CHECKSUM: [u8; 4] = [0x5d, 0xf6, 0xe0, 0xe2];


/// `Message` represents a message which contains `network_type` and `command` field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub network_type: NetworkType,
    pub command: Command,
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Version(version::VersionPayload),
    GetAddr,
    Addr(addr::AddrPayload),
}



pub struct MsgCodec;

impl Decoder for MsgCodec {
    type Item = Message;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        decode_message(src)
    }
}


impl Encoder for MsgCodec {
    type Item = Message;
    type Error = Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<()> {
        encode_message(item, dst)
    }
}
