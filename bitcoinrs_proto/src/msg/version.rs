use bitcoinrs_bytes::{Bytes, VarStr};

use commons::{NetAddrForVersionMsg, Services};
use super::MsgPayload;

const DEFAULT_VERSION: i32 = 70015;
const DEFAULT_SERVICES: Services = Services::NETWORK;

pub struct VersionMsg {
    pub version: i32,
    pub services: Services,
    pub timestamp: i64,
    pub addr_recv: NetAddrForVersionMsg,
    pub addr_from: NetAddrForVersionMsg,
    pub nonce: u64,
    pub user_agent: VarStr<'static>,
    pub start_height: i32,
    pub relay: bool,
}


impl Bytes for VersionMsg {
    fn length(&self) -> usize {
        self.version.length() + self.services.length()
            + self.timestamp.length() + self.addr_recv.length()
            + self.addr_from.length() + self.nonce.length()
            + self.user_agent.length() + self.start_height.length() + 1
    }

    fn write_to(&self, buf: &mut Vec<u8>) {
        self.version.to_le().write_to(buf);
        self.services.write_to(buf);
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
