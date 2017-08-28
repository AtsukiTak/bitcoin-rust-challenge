use bytes::{BytesMut, BufMut, BigEndian};

use message::MAINNET_START_STRING;

/// It means "getaddr"
const COMMAND_NAME: [u8; 12] = [0x67, 0x65, 0x74, 0x61, 0x64, 0x64, 0x72, 0, 0, 0, 0, 0];


pub fn encode_get_addr_msg(dst: &mut BytesMut) -> Result<()> {
    panic!();
}
