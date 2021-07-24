use crate::client::{Client, Error};
use crate::traits::{Packet, Writable, Readable};
use crate::client::Error::Refusal;
use crate::{MINECRAFT_VERSION, MINECRAFT_PROTOCOL_VERSION};

#[derive(Debug)]
pub struct RequestPacket {}

impl Packet for RequestPacket {
    fn id(&self) -> u32 { 0x00 }

    fn read(_: &mut dyn Readable) -> Result<Self, Error> where Self: Sized {
        Ok(RequestPacket {})
    }

    fn write(&self, _: &mut dyn Writable) -> Result<(), Error> {
        Err(Refusal)
    }

    fn act(&self, client: &mut Client) -> Result<(), Error> {
        client.write_packet(&ResponsePacket {
            json: format!(
                "{{\"version\":{{\"name\":\"Grimstone {}\",\"protocol\":{}1}},\"players\":{{\"max\":100,\"online\":50,\"sample\":[]}},\"description\":{{\"text\":\"{}\"}}}}",
                MINECRAFT_VERSION, MINECRAFT_PROTOCOL_VERSION,
                client.config.server_motd)
        })?;
        Ok(())
    }
}

pub struct ResponsePacket {
    json: String
}

impl Packet for ResponsePacket {
    fn id(&self) -> u32 { 0x00 }

    fn read(_: &mut dyn Readable) -> Result<Self, Error> where Self: Sized {
        Err(Refusal)
    }

    fn write(&self, output: &mut dyn Writable) -> Result<(), Error> {
        output.write_string(self.json.clone())?;
        Ok(())
    }

    fn act(&self, _: &mut Client) -> Result<(), Error> {
        Ok(())
    }
}

pub struct PingPongPacket {
    number: u64
}

impl Packet for PingPongPacket {
    fn id(&self) -> u32 { 0x01 }

    fn read(client: &mut dyn Readable) -> Result<Self, Error> where Self: Sized {
        Ok(Self {
            number: client.read_u64()?
        })
    }

    fn write(&self, client: &mut dyn Writable) -> Result<(), Error> {
        client.write_u64(self.number)?;
        Ok(())
    }

    fn act(&self, _: &mut Client) -> Result<(), Error> {
        Ok(())
    }
}
