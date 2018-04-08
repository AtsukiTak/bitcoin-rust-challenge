use std::net::SocketAddr;

use bitcoinrs_bytes::{Encodable, WriteBuf, endian::{i32_l, u64_l}};

use commons::{NetAddrForVersionMsg, Service, Services, Timestamp, VarStr};
use super::MsgPayload;

const USER_AGENT: &str = "bitcoinrs";

const DEFAULT_VERSION: i32 = 70015;

pub struct VersionMsg {
    version: i32,
    services: Services,
    timestamp: Timestamp,
    peer_addr: SocketAddr,
    self_addr: SocketAddr,
    nonce: u64,
    start_height: i32,
    relay: bool,
}

impl VersionMsg {
    pub fn new(peer_addr: SocketAddr, self_addr: SocketAddr, start_height: i32) -> VersionMsg {
        VersionMsg {
            version: DEFAULT_VERSION,
            services: Services::new(&[Service::Network]),
            timestamp: Timestamp::now(),
            peer_addr: peer_addr,
            self_addr: self_addr,
            nonce: 0,
            start_height: start_height,
            relay: false,
        }
    }
}

impl Encodable for VersionMsg {
    fn length(&self) -> usize {
        4 // version
        + 8 // services
        + 8 // timestamp
        + 26 // addr_recv
        + 26 // addr_from
        + 8 // nonce
        + VarStr(USER_AGENT).length()
        + 4 // start_height
        + 1 // relay
    }

    fn encode<W: WriteBuf>(&self, buf: &mut W) {
        i32_l::new(self.version).encode(buf);
        self.services.encode(buf);
        self.timestamp.encode(buf);
        NetAddrForVersionMsg::new(self.services, self.peer_addr).encode(buf);
        NetAddrForVersionMsg::new(self.services, self.self_addr).encode(buf);
        u64_l::new(self.nonce).encode(buf);
        VarStr(USER_AGENT).encode(buf);
        i32_l::new(self.start_height).encode(buf);
        (self.relay as u8).encode(buf);
    }
}

impl MsgPayload for VersionMsg {
    const COMMAND: &'static str = "version";
}
