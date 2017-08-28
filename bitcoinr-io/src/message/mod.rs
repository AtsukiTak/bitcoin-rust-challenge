mod decoder;

pub use self::decoder::decode_message;



pub const MAINNET_START_STRING: [u8; 4] = [0xf9, 0xbe, 0xb4, 0xd9];
pub const TESTNET3_START_STRING: [u8; 4] = [0x0b, 0x11, 0x09, 0x07];
pub const REGTEST_START_STRING: [u8; 4] = [0xfa, 0xbf, 0xb5, 0xda];

pub const SIZE_OF_HEADER: usize = 24;


pub enum Message {
    GetAddr,
}


struct Header {
    pub start_string: [u8; 4],
    pub command_name: [u8; 12],
    pub payload_size: u32,
    pub checksum: [u8; 4],
}
