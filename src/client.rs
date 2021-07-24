use std::collections::HashMap;
use std::fmt::{Display, Formatter, Debug};
use std::io::{Read, Write};
use std::net::TcpStream;

use crate::client::PacketState::Handshake;
use crate::traits::{Packet, Readable, Writable};
use crate::client::Error::{IoError, CannotReplace, InvalidPacketId};
use crate::buffer::Buffer;
use crate::config::ConcreteConfig;
use uuid::Uuid;
use uuid::Version::Random;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum PacketState {
    Handshake,
    Status,
    Login,
    Play
}

#[derive(Eq, PartialEq, Hash)]
pub struct PacketRef(pub PacketState, pub u32);

pub struct Client {
    pub stream: TcpStream,
    pub is_valid: bool,
    pub packets: HashMap<PacketRef, fn(&mut dyn Readable) -> Result<Box<dyn Packet>, Error>>,
    pub state: PacketState,
    pub config: ConcreteConfig,
    pub username: Option<String>,
    pub uuid: Option<Uuid>
}

pub struct RawPacket {
    pub len: u32,
    pub id: u32
}

#[derive(Debug)]
pub enum Error {
    Refusal,
    Disconnected,
    IoError(std::io::Error),
    CannotReplace,
    InvalidPacketId(PacketState, u32),
    StringTooLong(usize, usize, String)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("{:?}", self))?;
        Ok(())
    }
}

impl Client {
    pub fn new(stream: TcpStream, config: ConcreteConfig) -> Self {
        Self {
            stream,
            is_valid: true,
            packets: HashMap::new(),
            state: Handshake,
            config,
            username: None,
            uuid: Some(Uuid::from_u128(rand::random()))
        }
    }

    pub fn read_packet(&mut self) -> Result<Box<dyn Packet>, Error> {
        let len = self.read_var_int()?;
        let mut bytes = vec![0u8; len as usize];
        self.read(bytes.as_mut_slice())?;
        let mut buffer = Buffer::from(bytes.as_slice());
        let id = buffer.read_var_int()? as u32;
        let func_opt = self.packets.get(&PacketRef(self.state, id));
        if func_opt.is_none() {
            return Err(InvalidPacketId(self.state, id))
        }
        let pkt: Box<dyn Packet> = func_opt.unwrap()(&mut buffer)?;
        Ok(pkt)
    }

    pub fn write_packet(&mut self, packet: &dyn Packet) -> Result<usize, Error> {
        let mut buffer = Buffer::new();
        buffer.write_var_int(packet.id() as i32)?;
        packet.act(self)?;
        packet.write(&mut buffer)?;
        let mut size = self.write_var_int(buffer.bytes.len() as i32)?;
        size += self.write(&*buffer.bytes)?;
        Ok(size)
    }

    pub fn when(&mut self, state: PacketState, id: u32, func: fn(&mut dyn Readable) -> Result<Box<dyn Packet>, Error>) -> Result<(), Error>{
        match self.packets.insert(PacketRef(state, id), func) {
            Some(v) => {
                self.packets.insert(PacketRef(state, id), v);
                Err(CannotReplace)
            }
            None => Ok(())
        }
    }
}

impl Readable for Client {
    fn read(&mut self, array: &mut [u8]) -> Result<usize, Error> {
        let r = self.stream.read(array);
        match r {
            Ok(v) => Ok(v),
            Err(e) => Err(IoError(e))
        }
    }
}

impl Writable for Client {
    fn write(&mut self, array: &[u8]) -> Result<usize, Error> {
        match self.stream.write(array) {
            Ok(v) => Ok(v),
            Err(e) => Err(IoError(e))
        }
    }
}
