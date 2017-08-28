use bytes::{BytesMut, BufMut, BigEndian};

use super::get_addr::encode_get_addr_msg;

pub fn encode_message(msg: Message, dst: &mut BytesMut) -> Result<()> {
    match msg {
        Message::GetAdddr => encode_get_addr_msg(dst),
    }
}
