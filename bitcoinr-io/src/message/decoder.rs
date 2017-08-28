use bytes::{BytesMut, Bytes, BigEndian, Buf};

use std::io::{Cursor, SeekFrom, Seek};

use super::{SIZE_OF_HEADER, Message};
use error::*;



pub fn decode_message(src: &mut BytesMut) -> Result<Option<Message>> {
    if let Some(bytes) = extract_frame_bytes(src)? {
        panic!();
    } else {
        Ok(None)
    }
}



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
