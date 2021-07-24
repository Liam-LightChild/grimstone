//     C→S: Handshake with Next State set to 2 (login)
// [ ] C→S: Login Start
// [ ] S→C: Encryption Request
//     Client auth
// [ ] C→S: Encryption Response
// [ ] Server auth, both enable encryption
// [ ] S→C: Set Compression (optional)
// [ ] S→C: Login Success

use uuid::Uuid;
use crate::traits::{Packet, Readable, Writable};
use crate::client::{Error, Client};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem::transmute;
use std::mem::size_of;
use crate::client::Error::Refusal;
use crate::client::PacketState::Play;
use std::cell::Ref;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub struct StartLoginPacket {
    username: String
}

pub struct EndLoginPacket {
    uuid: Uuid,
    username: String
}

pub struct SetCompressionPacket {

}

impl Packet for StartLoginPacket {
    fn id(&self) -> u32 { 0x00 }

    fn read(input: &mut dyn Readable) -> Result<Self, Error> where Self: Sized {
        Ok(Self {
            username: input.read_string(16)?
        })
    }

    fn write(&self, _: &mut dyn Writable) -> Result<(), Error> {
        Err(Refusal)
    }

    fn act(&self, client: &mut Client) -> Result<(), Error> {
        let mut h = DefaultHasher::new();
        client.username = Some(self.username.clone());
        client.username.hash(&mut h);
        let mut rng = ChaCha8Rng::seed_from_u64(h.finish());
        client.uuid = Some(Uuid::from_u128(rng.gen()));

        client.write_packet(&EndLoginPacket {
            uuid: client.uuid.clone().unwrap(),
            username: client.username.clone().unwrap()
        })?;

        log::info!("{} as {}/{} has joined",
            client.stream.local_addr().unwrap(),
            client.username.clone().unwrap(),
            client.uuid.clone().unwrap());

        Ok(())
    }
}

impl Packet for EndLoginPacket {
    fn id(&self) -> u32 { 0x02 }

    fn read(_: &mut dyn Readable) -> Result<Self, Error> where Self: Sized {
        Err(Refusal)
    }

    fn write(&self, output: &mut dyn Writable) -> Result<(), Error> {
        output.write_u128(self.uuid.as_u128())?;
        output.write_string(self.username.clone())?;
        Ok(())
    }

    fn act(&self, client: &mut Client) -> Result<(), Error> {
        log::info!("State swap occurring; {:?} -> {:?}", client.state, Play);
        client.state = Play;
        Ok(())
    }
}
