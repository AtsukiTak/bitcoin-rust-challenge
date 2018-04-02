use commons::*;
use NetworkType;

pub trait MsgPayload: AsBytes {
    const COMMAND: &'static str;

    fn to_msg_bytes(&self, network: NetworkType) -> Vec<u8> {
        let mut buf = Vec::with_capacity(21 + self.length());

        // Write magic_number
        lu32(network.magic_num()).write_to(&mut buf);

        // Write command_string
        write_command(Self::COMMAND, &mut buf);

        // Write payload_size
        lu32(self.length() as u32).write_to(&mut buf);

        // Write checksum
        // TODO

        // Write payload
        self.write_to(&mut buf);

        buf
    }
}

fn write_command(command: &str, buf: &mut Vec<u8>) {
    assert!(command.len() <= 11);

    let mut bytes: [u8; 12] = [0; 12];
    bytes.copy_from_slice(command.as_bytes());
    buf.extend_from_slice(&bytes);
}

pub struct VersionMsg {
    version: li32,
    services: lu64,
    timestamp: li64,
    addr_recv: NetAddr,
    addr_from: NetAddr,
    nonce: lu64,
    user_agent: VarStr<'static>,
    start_height: li32,
    relay: bool,
}

impl AsBytes for VersionMsg {
    fn length(&self) -> usize {
        self.version.length() + self.services.length() + self.timestamp.length()
            + self.addr_from.length() + self.addr_from.length() + self.nonce.length()
            + self.user_agent.length() + self.start_height.length() + 1
    }

    fn write_to(&self, buf: &mut Vec<u8>) {
        self.version.write_to(buf);
        self.services.write_to(buf);
        self.timestamp.write_to(buf);
        self.addr_recv.write_to(buf);
        self.addr_from.write_to(buf);
        self.nonce.write_to(buf);
        self.user_agent.write_to(buf);
        self.start_height.write_to(buf);
        buf.push(self.relay as u8);
    }
}
