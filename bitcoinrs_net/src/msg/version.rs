use std::net::SocketAddr;

use bitcoinrs_bytes::{Encodable, WriteBuf, endian::{i32_l, u64_l}};

use commons::{NetAddrForVersionMsg, Service, Services, Timestamp, VarStr};
use super::MsgPayload;

const DEFAULT_USER_AGENT: &str = "bitcoinrs";

const DEFAULT_VERSION: i32 = 70015;

#[derive(Debug, Clone, Copy)]
pub struct VersionMsgPayload {
    version: i32,
    services: Services,
    peer_addr: SocketAddr,
    self_addr: SocketAddr,
    nonce: u64,
    user_agent: &'static str,
    start_height: i32,
    relay: bool,
}

impl VersionMsgPayload {
    /// Create a new `VersionMsgPayload` struct.
    /// Initialize some field with default value.
    ///
    /// # Default fields
    /// - version : 70015
    /// - services : NODE_NETWORK
    /// - nonce : 0
    /// - user_agent : bitcoinrs
    /// - start_height : 0
    /// - relay : false
    pub fn new(peer_addr: SocketAddr, self_addr: SocketAddr) -> VersionMsgPayload {
        VersionMsgPayload {
            version: DEFAULT_VERSION,
            services: Services::new(&[Service::Network]),
            peer_addr: peer_addr,
            self_addr: self_addr,
            nonce: 0, // If this value is 0, nonce field is ignored.
            user_agent: DEFAULT_USER_AGENT,
            start_height: 0,
            relay: false,
        }
    }

    pub fn version(&mut self, ver: i32) -> &mut Self {
        self.version = ver;
        self
    }

    pub fn services(&mut self, services: Services) -> &mut Self {
        self.services = services;
        self
    }

    pub fn nonce(&mut self, nonce: u64) -> &mut Self {
        self.nonce = nonce;
        self
    }

    pub fn user_agent(&mut self, ua: &'static str) -> &mut Self {
        self.user_agent = ua;
        self
    }

    pub fn start_height(&mut self, n: i32) -> &mut Self {
        self.start_height = n;
        self
    }

    pub fn relay(&mut self, relay: bool) -> &mut Self {
        self.relay = relay;
        self
    }
}

impl Encodable for VersionMsgPayload {
    fn length(&self) -> usize {
        4 // version
        + 8 // services
        + 8 // timestamp
        + 26 // addr_recv
        + 26 // addr_from
        + 8 // nonce
        + VarStr(self.user_agent).length()
        + 4 // start_height
        + 1 // relay
    }

    fn encode<W: WriteBuf>(&self, buf: &mut W) {
        let chained = i32_l::new(self.version)
            .chain(self.services)
            .chain(Timestamp::now())
            .chain(NetAddrForVersionMsg::new(self.services, self.peer_addr))
            .chain(NetAddrForVersionMsg::new(self.services, self.self_addr))
            .chain(u64_l::new(self.nonce))
            .chain(VarStr(self.user_agent))
            .chain(i32_l::new(self.start_height))
            .chain(self.relay as u8);
        buf.write(chained);
    }
}

impl MsgPayload for VersionMsgPayload {
    const COMMAND: &'static str = "version";
}
