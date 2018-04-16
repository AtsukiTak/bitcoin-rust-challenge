use std::net::ToSocketAddr;
use std::io::Error as IoError;

pub fn open_connection(addr: SocketAddr, net_type: NetworkType) -> Result<Handshaking, IoError> {
    let mut socket = TcpStream::connect(addr.clone())?;

    let local_addr = socket.local_addr()?;
    let remote_addr = addr;
    let version_msg = VersionMsgPayload::new(peer_addr, local_addr).into_msg(net_type);

    Ok(Handshaking {
        version_msg: version_msg,
    })
}

pub struct Handshaking {
    version_msg: VersionMsg,
    socket: TcpStream,
}

impl Handshaking {
    pub fn version_msg_mut(&mut self) -> &mut VersionMsg {
        &mut self.version_msg
    }

    pub fn send_version_msg(&mut self) {
        self.socket.write_all(self.version_msg.to_vec().as_slice())?;
        let _verack = self.socket.read::<VerackMsg>().unwrap();
    }
}

pub struct Socket {
    socket: TcpStream
    read_buf: Buffer,
}

impl Socket {
    pub fn new(socket: TcpStream) -> Result<Socket, IoError> {
        Socket {
            socket: socket,
            read_buf: Buffer::new(),
        }
    }

    pub fn send_msg<P: Payload>(&self, msg: Msg<P>) -> Result<(), IoError> {
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
    }

    fn recv_msg_sync<P: Payload>(&mut self) -> Result<Msg<P>, IoError> {
        self.read_to_buffer()?;
    }
}
