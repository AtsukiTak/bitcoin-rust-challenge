extern crate bitcoinrs_bytes;
extern crate bitcoinrs_net;

use std::net::*;
use std::io::{Read, Write};

use bitcoinrs_bytes::Encodable;

use bitcoinrs_net::{NetworkType, msg::{Msg, VersionMsgPayload}};

fn main() {
    let mut socket = TcpStream::connect("138.201.55.219:8333").unwrap();
    println!("connected");

    let ver_msg = {
        let peer_addr = "138.201.55.219:8333".parse().unwrap();
        let local_addr = socket.local_addr().unwrap();
        println!("local addr : {:?}", local_addr);
        let payload = VersionMsgPayload::new(peer_addr, local_addr);
        Msg::new(NetworkType::Main, payload)
    };

    socket.write_all(ver_msg.to_vec().as_slice()).unwrap();
    socket.flush().unwrap();

    println!("Sent version msg");

    let mut buf = [0];
    socket.read_exact(&mut buf).unwrap();
}
