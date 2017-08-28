use bytes::{BytesMut, Bytes, BigEndian, Buf};

use std::io::{Cursor, SeekFrom, Seek};

use super::{SIZE_OF_HEADER, Message};
use net::NetworkType;
use error::*;



pub fn decode_message(src: &mut BytesMut) -> Result<Option<Message>> {
    if let Some(bytes) = extract_frame_bytes(src)? {
        panic!();
    } else {
        Ok(None)
    }
}



/// Extract bytes which exactry represents one message.
fn extract_frame_bytes(src: &mut BytesMut) -> Result<Option<Bytes>> {
    if src.len() < SIZE_OF_HEADER {
        return Ok(None);
    } else {
        let payload_size = {
            let mut cursor = Cursor::new(&src);
            cursor.seek(SeekFrom::Current(16_i64))?;
            cursor.get_u32::<BigEndian>() as usize
        };

        if src.len() < SIZE_OF_HEADER + payload_size {
            return Ok(None);
        } else {
            return Ok(Some(src.split_to(SIZE_OF_HEADER + payload_size).freeze()));
        }
    }
}



/// This function reads network type from src.
/// You must pass Bytes which starts with network start string.
/// # Panics
/// when size of src is less than 4.
fn read_network_type(src: &Bytes) -> Result<NetworkType> {
    NetworkType::from_start_string([src[0], src[1], src[2], src[3]])
}
