extern crate bitcoinrs_proto;

use std::net::*;

fn main() {
    let socket = TcpStream::connect("138.201.55.219:8333").unwrap();
}
