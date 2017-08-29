use bytes::{Bytes, BytesMut, BigEndian, BufMut};

use std::net::SocketAddr;

use error::*;


/// It means "version"
pub const COMMAND_NAME: [u8; 12] = [b'v', b'e', b'r', b's', b'i', b'o', b'n', 0, 0, 0, 0, 0];

const MAX_SIZE_OF_VERSION_PAYLOAD: usize = 310;

pub const UNKNOWN_IP_ADDR: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 127, 0, 0, 1];



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionPayload {
    // The highest protocol version understood by the transmitting node. See the protocol version
    // section ( https://bitcoin.org/en/developer-reference#protocol-versions ).
    version: i32,

    // The services supported by the transmitting node encoded as a bitfield. See the list of
    // service codes below.
    services: u64,

    // The current Unix epoch time according to the transmitting node’s clock. Because nodes will
    // reject blocks with timestamps more than two hours in the future, this field can help other
    // nodes to determine that their clock is wrong.
    timestamp: i64,

    // Added in protocol version 106.
    //
    // The services supported by the receiving node as perceived by the transmitting node. Same
    // format as the ‘services’ field above. Bitcoin Core will attempt to provide accurate
    // information. BitcoinJ will, by default, always send 0.
    addr_recv_services: u64,

    // Added in protocol version 106.
    //
    // ## Ip address
    // The IPv6 address of the receiving node as perceived by the transmitting node in big endian
    // byte order. IPv4 addresses can be provided as IPv4-mapped IPv6 addresses. Bitcoin Core will
    // attempt to provide accurate information. BitcoinJ will, by default, always return
    // ::ffff:127.0.0.1
    //
    // ## Port number
    // The port number of the receiving node as perceived by the transmitting node in big endian
    // byte order
    addr_recv: SocketAddr,

    // The services supported by the transmitting node. Should be identical to the ‘services’
    // field above.
    addr_trans_services: u64,

    // ## Ip address
    // The IPv6 address of the transmitting node in big endian byte order. IPv4 addresses can be
    // provided as IPv4-mapped IPv6 addresses. Set to ::ffff:127.0.0.1 if unknown.
    //
    // ## Port number
    // The port number of the transmitting node in big endian byte order.
    addr_trans: SocketAddr,

    // A random nonce which can help a node detect a connection to itself. If the nonce is 0, the
    // nonce field is ignored. If the nonce is anything else, a node should terminate the
    // connection on receipt of a version message with a nonce it previously sent.
    nonce: u64,

    // Renamed in protocol version 60000.
    //
    // User agent as defined by BIP14. Previously called subVer.
    user_agent: Option<&'static str>,

    // The height of the transmitting node’s best block chain or, in the case of an SPV client,
    // best block header chain.
    start_height: i32,

    // Added in protocol version 70001 as described by BIP37.
    //
    // Transaction relay flag. If 0x00, no inv messages or tx messages announcing new transactions
    // should be sent to this client until it sends a filterload message or filterclear message. If
    // the relay field is not present or is set to 0x01, this node wants inv messages and tx
    // messages announcing new transactions.
    //
    // This field is optional.
    relay: Option<bool>,
}



pub fn decode(payload: Bytes) -> Result<VersionPayload> {
    panic!()
}



pub fn encode(version: VersionPayload) -> Result<Bytes> {
    let mut buf = BytesMut::with_capacity(MAX_SIZE_OF_VERSION_PAYLOAD);

    buf.put_i32::<BigEndian>(version.version);
    buf.put_u64::<BigEndian>(version.services);
    buf.put_i64::<BigEndian>(version.timestamp);

    buf.put_u64::<BigEndian>(version.addr_recv_services);
    encode_addr(version.addr_recv, &mut buf);

    buf.put_u64::<BigEndian>(version.addr_trans_services);
    encode_addr(version.addr_trans, &mut buf);

    buf.put_u64::<BigEndian>(version.nonce);

    if let Some(user_agent) = version.user_agent {
        assert!(user_agent.len() < 0xFD as usize); // `user_agent` field must be less than 0xFD.
        buf.put_u8(user_agent.len() as u8);
        buf.put_slice(&user_agent.as_bytes());
    }

    buf.put_i32::<BigEndian>(version.start_height);

    if let Some(relay) = version.relay {
        buf.put_u8(relay as u8);
    }

    Ok(buf.freeze())
}


fn encode_addr(addr: SocketAddr, dst: &mut BytesMut) {
    match addr {
        SocketAddr::V4(ipv4) => {
            dst.put_slice(&ipv4.ip().to_ipv6_mapped().octets());
            dst.put_u16::<BigEndian>(ipv4.port());
        }
        SocketAddr::V6(ipv6) => {
            dst.put_slice(&ipv6.ip().octets());
            dst.put_u16::<BigEndian>(ipv6.port());
        }
    }
}
