extern crate bitcoinrs_bytes;
extern crate bitcoinrs_net;

use std::env::args;

use bitcoinrs_net::{NetworkType, socket::open_connection};

fn main() {
    let first_arg = args().skip(1).next().unwrap();
    let handshake = open_connection(first_arg.parse().unwrap(), NetworkType::Main).unwrap();

    println!("connected");

    handshake.send_version_msg().unwrap();

    println!("Sent version msg");
}
