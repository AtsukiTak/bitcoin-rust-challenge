extern crate bitcoinrs_bytes;
extern crate bitcoinrs_crypto;

pub mod commons;
pub mod msg;

const MAGIC_MAIN: u32 = 0xd9b4bef9;
const MAGIC_TEST3: u32 = 0x0709110b;

#[derive(Clone, Copy, Debug)]
pub enum NetworkType {
    Main,
    Testnet3,
}

impl NetworkType {
    pub fn magic_num(&self) -> u32 {
        match *self {
            NetworkType::Main => MAGIC_MAIN,
            NetworkType::Testnet3 => MAGIC_TEST3,
        }
    }

    pub fn from_magic_num(magic: u32) -> Option<NetworkType> {
        match magic {
            MAGIC_MAIN => Some(NetworkType::Main),
            MAGIC_TEST3 => Some(NetworkType::Testnet3),
            _ => None,
        }
    }
}
