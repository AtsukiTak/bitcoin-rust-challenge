use bytes::{BytesMut, Bytes, BufMut, BigEndian};

use sha2::{Sha256, Digest};

use super::{SIZE_OF_HEADER, Message, Command, EMPTY_STRING_CHECKSUM};
use super::command::{get_addr, addr, version};
use error::*;


pub fn encode_message(msg: Message, dst: &mut BytesMut) -> Result<()> {

    println!("Encode msg : {:?}", msg);

    // Get each command name and payload bytes.
    let (command_name, payload) = get_command_name_and_payload(msg.command)?;

    let payload_size = payload.len();

    // Write message header.
    dst.reserve(SIZE_OF_HEADER);
    dst.put(msg.network_type.start_string().as_ref());
    dst.put(command_name.as_ref());
    dst.put_u32::<BigEndian>(payload_size as u32);
    if payload_size == 0 {
        dst.put(EMPTY_STRING_CHECKSUM.as_ref());
    } else {
        // Calculate Sha256 hash
        let hashed_once = Sha256::digest(payload.as_ref());
        let hashed_twice = Sha256::digest(&hashed_once.as_ref());
        dst.put(hashed_twice[0..4].as_ref());
    }

    // Write payload.
    dst.reserve(payload_size);
    dst.put(payload);

    println!("Finish encodeing : {:?}", dst);

    Ok(())
}



fn get_command_name_and_payload(command: Command) -> Result<([u8; 12], Bytes)> {
    match command {
        Command::Version(version) => Ok((version::COMMAND_NAME, version::encode(version)?)),
        Command::GetAddr => Ok((get_addr::COMMAND_NAME, get_addr::encode()?)),
        Command::Addr(addr) => Ok((addr::COMMAND_NAME, addr::encode(addr)?)),
    }
}
