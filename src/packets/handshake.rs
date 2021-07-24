use crate::client::{Client, Error, PacketState};
use crate::traits::{Packet, Readable, Writable};
use crate::client::Error::Refusal;
use std::cell::Ref;

#[derive(Debug)]
pub struct HandshakePacket {
    proto_version: u32,
    address: String,
    port: u16,
    next: PacketState
}

impl Packet for HandshakePacket {
    fn id(&self) -> u32 { 0x00 }

    fn read(client: &mut dyn Readable) -> Result<Self, Error> where Self: Sized {
        Ok(Self {
            proto_version: client.read_var_int()? as u32,
            address: client.read_string(255)?,
            port: client.read_u16()?,
            next: match client.read_var_int()? {
                1 => PacketState::Status,
                2 => PacketState::Login,
                v => panic!("Invalid next value {}", v)
            }
        })
    }

    fn write(&self, _: &mut dyn Writable) -> Result<(), Error> {
        Err(Refusal)
    }

    fn act(&self, client: &mut Client) -> Result<(), Error> {
        log::info!("State swap occurring; {:?} -> {:?}", client.state, self.next);
        client.state = self.next;
        Ok(())
    }
}
