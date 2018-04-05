use bitcoinrs_bytes::{Bytes, VarStr};

use commons::*;
use super::MsgPayload;

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

impl MsgPayload for VersionMsg {
    const COMMAND: &'static str = "version";
}