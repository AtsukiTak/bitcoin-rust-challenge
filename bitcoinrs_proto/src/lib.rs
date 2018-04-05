extern crate bitcoinrs_bytes;
extern crate bitcoinrs_crypto;

pub mod commons;
pub mod msg;

#[derive(Clone, Copy, Debug)]
pub enum NetworkType {
    Main,
    Testnet3,
}

impl NetworkType {
    pub fn magic_num(&self) -> u32 {
        match *self {
            NetworkType::Main => 0xd9b4bef9,
            NetworkType::Testnet3 => 0x0709110b,
        }
    }
}
