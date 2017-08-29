use bytes::{BytesMut, Bytes, BigEndian, Buf};

use sha2::{Sha256, Digest};

use std::io::{Cursor, SeekFrom, Seek};

use super::command::{get_addr, addr};
use message::{Command, Message, SIZE_OF_HEADER, EMPTY_STRING_CHECKSUM};
use net::NetworkType;
use error::*;



pub fn decode_message(src: &mut BytesMut) -> Result<Option<Message>> {

    println!("Decode bytes : {:?}", src);

    if let Some(bytes) = extract_frame_bytes(src)? {

        let net_type = read_network_type(&bytes.slice(0, 4))?;
        let payload = bytes.slice_from(SIZE_OF_HEADER);

        check_checksum(&bytes.slice(20, 20 + 4), &payload)?;

        let command_name = read_command_name(&bytes.slice(4, 4 + 12));
        let command = read_command(command_name, payload)?;

        Ok(Some(Message {
            network_type: net_type,
            command: command,
        }))

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



/// This function reads command name from src.
/// You must pass Bytes which starts with command name string.
/// # Panics
/// when size of src is less than 12.
fn read_command_name(src: &Bytes) -> [u8; 12] {
    [src[0], src[1], src[2], src[3], src[4], src[5], src[6], src[7], src[8], src[9], src[10],
     src[11]]
}



/// Check whether payload is valid or not.
/// You must pass Bytes which starts with checksum as `checksum`,
/// and another Bytes which represents payload as `payload`.
/// # Panics
/// when size of checksum is less than 4.
fn check_checksum(checksum: &Bytes, payload: &Bytes) -> Result<()> {

    let calculated_checksum = if payload.len() == 0 {
        EMPTY_STRING_CHECKSUM
    } else {
        let hashed_once = Sha256::digest(payload.as_ref());
        let hashed_twice = Sha256::digest(hashed_once.as_ref());
        [hashed_twice[0], hashed_twice[1], hashed_twice[2], hashed_twice[3]]
    };

    if calculated_checksum == checksum[0..4] {
        Ok(())
    } else {
        Err(ErrorKind::ChecksumDoesNotAccord.into())
    }
}



fn read_command(command_name: [u8; 12], payload: Bytes) -> Result<Command> {
    match command_name {
        addr::COMMAND_NAME => Ok(Command::Addr(addr::decode(payload)?)),
        get_addr::COMMAND_NAME => Ok(Command::GetAddr),

        other => Err(ErrorKind::InvalidCommandName(other).into()),
    }
}
