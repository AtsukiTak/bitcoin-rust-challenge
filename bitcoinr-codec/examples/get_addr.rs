extern crate bitcoinr_codec;
extern crate tokio_core;
extern crate tokio_io;
extern crate futures;

use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_io::AsyncRead;

use futures::{Future, Sink, Stream};

use bitcoinr_codec::{MsgCodec, Message, NetworkType, Command};
use bitcoinr_codec::error::*;


pub fn main() {

    let mut core = Core::new().unwrap();

    let get_addr_msg = Message {
        network_type: NetworkType::Main,
        command: Command::GetAddr,
    };

    let future = TcpStream::connect(&"46.23.87.218:8333".parse().unwrap(), &core.handle())
        .map_err(|e| Error::from(e))
        .map(|stream| stream.framed(MsgCodec))
        .and_then(|framed| framed.send(get_addr_msg).map_err(|e| Error::from(e)));

    let res = core.run(future).unwrap();

    core.run(res.for_each(|data| Ok(println!("{:?}", data))));
}
