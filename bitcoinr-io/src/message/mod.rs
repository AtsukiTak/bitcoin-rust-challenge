mod decoder;
mod encoder;
pub(self) mod command;

pub use self::decoder::decode_message;
pub use self::encoder::encode_message;

use net::NetworkType;
use self::command::addr;


pub const SIZE_OF_HEADER: usize = 24;

pub const EMPTY_STRING_CHECKSUM: [u8; 4] = [0x5d, 0xf6, 0xe0, 0xe2];


/// `Message` represents a message which contains `network_type` and `command` field.
pub struct Message {
    network_type: NetworkType,
    command: Command,
}


pub enum Command {
    GetAddr,
    Addr(addr::AddrPayload),
}
