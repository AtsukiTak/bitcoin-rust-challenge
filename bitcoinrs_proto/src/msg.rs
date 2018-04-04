use bitcoinrs_bytes::{Bytes, VarStr};

use commons::*;
use NetworkType;

pub trait MsgPayload: Bytes {
    const COMMAND: &'static str;

    fn to_msg_bytes(&self, network: NetworkType) -> Vec<u8> {
        let mut buf = Vec::with_capacity(21 + self.length());

        // Write magic_number
        network.magic_num().to_le().write_to(&mut buf);

        // Write command_string
        write_command(Self::COMMAND, &mut buf);

        // Write payload_size
        (self.length() as u32).to_le().write_to(&mut buf);

        // Write checksum
        // TODO

        // Write payload
        self.write_to(&mut buf);

        buf
    }
}

fn write_command(command: &str, buf: &mut Vec<u8>) {
    assert!(command.len() <= 11);

    let mut bytes: [u8; 12] = [0; 12];
    bytes.copy_from_slice(command.as_bytes());
    buf.extend_from_slice(&bytes);
}

pub struct VersionMsg {
    version: i32,
    services: u64,
    timestamp: i64,
    addr_recv: NetAddr,
    addr_from: NetAddr,
    nonce: u64,
    user_agent: VarStr<'static>,
    start_height: i32,
    relay: bool,
}

impl Bytes for VersionMsg {
    fn length(&self) -> usize {
        self.version.to_le().length() + self.services.to_le().length()
            + self.timestamp.to_le().length() + self.addr_recv.length()
            + self.addr_from.length() + self.nonce.to_le().length()
            + self.user_agent.length() + self.start_height.to_le().length() + 1
    }

    fn write_to(&self, buf: &mut Vec<u8>) {
        self.version.to_le().write_to(buf);
        self.services.to_le().write_to(buf);
        self.timestamp.to_le().write_to(buf);
        self.addr_recv.write_to(buf);
        self.addr_from.write_to(buf);
        self.nonce.to_le().write_to(buf);
        self.user_agent.write_to(buf);
        self.start_height.to_le().write_to(buf);
        buf.push(self.relay as u8);
    }
}
