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
        let chained = i32_l::new(self.version)
            .chain(self.services)
            .chain(self.timestamp)
            .chain(NetAddrForVersionMsg::new(self.services, self.peer_addr))
            .chain(NetAddrForVersionMsg::new(self.services, self.self_addr))
            .chain(u64_l::new(self.nonce))
            .chain(VarStr(USER_AGENT))
            .chain(i32_l::new(self.start_height))
            .chain(self.relay as u8);
        buf.write(chained);
    }
}

impl MsgPayload for VersionMsg {
    const COMMAND: &'static str = "version";
}
