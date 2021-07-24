use crate::client::{Client, PacketState, Error};
use crate::client::PacketState::{Handshake, Status, Login};
use crate::traits::Packet;
use crate::packets::status::{RequestPacket, PingPongPacket};
use crate::packets::handshake::HandshakePacket;
use crate::packets::login::StartLoginPacket;

pub mod handshake;
pub mod status;
mod login;

impl Client {
    fn register<T: 'static + Packet>(&mut self, state: PacketState, id: u32) -> Result<(), Error> {
        self.when(state, id, |i| Ok(Box::new(T::read(i)?)))?;
        Ok(())
    }
}

pub fn register(client: &mut Client) -> Result<(), ()> {
    client.register::<HandshakePacket>(Handshake, 0x00);

    client.register::<RequestPacket>(Status, 0x00);
    client.register::<PingPongPacket>(Status, 0x01);

    client.register::<StartLoginPacket>(Login, 0x00);

    Ok(())
}