use error::*;


pub const MAINNET_START_STRING: [u8; 4] = [0xf9, 0xbe, 0xb4, 0xd9];
pub const TESTNET3_START_STRING: [u8; 4] = [0x0b, 0x11, 0x09, 0x07];
pub const REGTEST_START_STRING: [u8; 4] = [0xfa, 0xbf, 0xb5, 0xda];


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkType {
    Main,
    Test,
    RegTest,
}


impl NetworkType {
    pub fn start_string(&self) -> [u8; 4] {
        match self {
            &NetworkType::Main => MAINNET_START_STRING,
            &NetworkType::Test => TESTNET3_START_STRING,
            &NetworkType::RegTest => REGTEST_START_STRING,
        }
    }

    pub fn from_start_string(src: [u8; 4]) -> Result<NetworkType> {
        match src {
            MAINNET_START_STRING => Ok(NetworkType::Main),
            TESTNET3_START_STRING => Ok(NetworkType::Test),
            REGTEST_START_STRING => Ok(NetworkType::RegTest),
            other => Err(ErrorKind::InvalidStartString(other).into()),
        }
    }
}
