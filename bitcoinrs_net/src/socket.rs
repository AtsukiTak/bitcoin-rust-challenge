use std::net::{SocketAddr, TcpStream};
use std::io::{Cursor, Error as IoError, Read, Write};

use bitcoinrs_bytes::{buffer::Buffer, decode::ReadBuffer, encode::{Encodable, WriteBuffer}};

use {NetworkType, msg::{Msg, MsgPayload, VersionMsg, VersionMsgPayload}};

pub fn open_connection(
    remote_addr: SocketAddr,
    net_type: NetworkType,
) -> Result<Handshaking, IoError> {
    let socket = TcpStream::connect(remote_addr.clone())?;

    let local_addr = socket.local_addr()?;
    let version_msg = VersionMsgPayload::new(remote_addr, local_addr).into_msg(net_type);

    Ok(Handshaking {
        version_msg: version_msg,
        socket: Socket::new(socket),
    })
}

pub struct Handshaking {
    version_msg: VersionMsg,
    socket: Socket,
}

impl Handshaking {
    pub fn version_msg_mut(&mut self) -> &mut VersionMsg {
        &mut self.version_msg
    }
}

pub struct Socket {
    socket: TcpStream,
    read_buf: Buffer,
}

impl Socket {
    pub fn new(socket: TcpStream) -> Socket {
        Socket {
            socket: socket,
            read_buf: Buffer::new(),
        }
    }

    pub fn send_msg<P: MsgPayload>(&self, msg: Msg<P>) -> Result<(), IoError> {
        (&self.socket).write_all(msg.to_vec().as_slice())
    }

    fn read_to_buffer(&mut self) -> Result<(), IoError> {
        const TMP_BUF_SIZE: usize = 128;
        let mut tmp_buf = [0; TMP_BUF_SIZE];

        loop {
            let n = self.socket.read(&mut tmp_buf)?;
            self.read_buf.write_bytes(&tmp_buf[..n]);
            if n < TMP_BUF_SIZE {
                break;
            }
        }
        Ok(())
    }

    pub fn recv_msg_sync<P: MsgPayload>(&mut self) -> Result<Option<Msg<P>>, IoError> {
        self.read_to_buffer()?;

        let (msg, dropped) = {
            let mut read_buf = Cursor::new(&self.read_buf);
            let msg = match <Cursor<&Buffer> as ReadBuffer>::read::<Msg<P>>(&mut read_buf) {
                Ok(msg) => msg,
                Err(_) => return Ok(None),
            };
            let dropped = read_buf.position() as usize;
            (msg, dropped)
        };

        self.read_buf.drop_front(dropped);

        Ok(Some(msg))
    }
}
