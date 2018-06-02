use std::net::Ipv6Addr;

use bitcoinrs_bytes::decode::{Decodable, DecodeError, ReadBuffer};
use bitcoinrs_bytes::encode::{Encodable, WriteBuffer};
use bitcoinrs_bytes::endian::{i32_l, u64_l};

use super::common_types::{NetAddrForVersionMsg, Service, Services, Timestamp, VarStr};
use super::MsgPayload;

const DEFAULT_USER_AGENT: &str = "bitcoinrs";

const DEFAULT_VERSION: i32 = 70015;

#[derive(Debug, Clone)]
pub struct VersionMsgPayload {
    version: i32,
    services: Services,
    timestamp: Timestamp,
    remote_ip: Ipv6Addr,
    remote_port: u16,
    local_ip: Ipv6Addr,
    local_port: u16,
    nonce: u64,
    user_agent: VarStr,
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
    /// - timestamp : [current timestamp]
    /// - remote_ip : ::ffff:127:0.0.1
    /// - remote_port : 8333
    /// - local_ip : ::ffff:127:0:0:1
    /// - local_port : 8333
    /// - nonce : 0
    /// - user_agent : bitcoinrs
    /// - start_height : 0
    /// - relay : false
    pub fn new() -> VersionMsgPayload {
        VersionMsgPayload {
            version: DEFAULT_VERSION,
            services: Services::new(&[Service::Network]),
            timestamp: Timestamp::now(),
            remote_ip: Ipv6Addr::new(0, 0, 0, 0xffff, 127, 0, 0, 1),
            remote_port: 8331,
            local_ip: Ipv6Addr::new(0, 0, 0, 0xffff, 127, 0, 0, 1),
            local_port: 8331,
            nonce: 0, // If this value is 0, nonce field is ignored.
            user_agent: VarStr(DEFAULT_USER_AGENT.into()),
            start_height: 0,
            relay: false,
        }
    }

    pub fn set_version(&mut self, ver: i32) -> &mut Self {
        self.version = ver;
        self
    }

    pub fn set_services(&mut self, services: Services) -> &mut Self {
        self.services = services;
        self
    }

    pub fn set_timestamp(&mut self, timestamp: Timestamp) -> &mut Self {
        self.timestamp = timestamp;
        self
    }

    pub fn set_nonce(&mut self, nonce: u64) -> &mut Self {
        self.nonce = nonce;
        self
    }

    pub fn set_user_agent(&mut self, ua: String) -> &mut Self {
        self.user_agent = VarStr(ua);
        self
    }

    pub fn set_start_height(&mut self, n: i32) -> &mut Self {
        self.start_height = n;
        self
    }

    pub fn set_relay(&mut self, relay: bool) -> &mut Self {
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
        + (&self.user_agent).length()
        + 4 // start_height
        + 1 // relay
    }

    fn encode<W: WriteBuffer>(&self, buf: &mut W) {
        buf.write(i32_l::new(self.version));
        buf.write(self.services);
        buf.write(self.timestamp);
        buf.write(NetAddrForVersionMsg::new(
            self.services,
            self.remote_ip,
            self.remote_port,
        ));
        buf.write(NetAddrForVersionMsg::new(
            self.services,
            self.local_ip,
            self.local_port,
        ));
        buf.write(u64_l::new(self.nonce));
        buf.write(&self.user_agent);
        buf.write(i32_l::new(self.start_height));
        buf.write(self.relay as u8);
    }
}

impl Decodable for VersionMsgPayload {
    fn decode<R: ReadBuffer>(bytes: &mut R) -> Result<VersionMsgPayload, DecodeError> {
        let version = bytes.read::<i32_l>()?.value();
        let services = bytes.read::<Services>()?;
        let timestamp = bytes.read::<Timestamp>()?;

        let remote_addr = bytes.read::<NetAddrForVersionMsg>()?;
        let local_addr = bytes.read::<NetAddrForVersionMsg>()?;

        let nonce = bytes.read::<u64_l>()?.value();
        let user_agent = bytes.read::<VarStr>()?;
        let start_height = bytes.read::<i32_l>()?.value();
        let relay = bytes.read::<u8>()? == 1;

        Ok(VersionMsgPayload {
            version: version,
            services: services,
            timestamp: timestamp,
            remote_ip: remote_addr.ip,
            remote_port: remote_addr.port,
            local_ip: local_addr.ip,
            local_port: local_addr.port,
            nonce: nonce,
            user_agent: user_agent,
            start_height: start_height,
            relay: relay,
        })
    }
}

impl MsgPayload for VersionMsgPayload {
    const COMMAND_BYTES: [u8; 12] = [0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0, 0, 0, 0, 0];
}
