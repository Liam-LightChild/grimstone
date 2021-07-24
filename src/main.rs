use std::net::TcpListener;
use std::process::exit;
use std::sync::{Arc, mpsc};
use std::thread;

use grimstone::traits::Packet;
use grimstone::client::{Client, PacketRef, Error};
use grimstone::config::{Config, ConcreteConfig};
use grimstone::packets;
use simple_logger::SimpleLogger;
use log::LevelFilter;
use grimstone::server::Server;
use grimstone::single::SingleWorldFile;
use grimstone::world::{WorldSyncer, World, ChunkContainer, Chunk};
use std::any::Any;
use std::borrow::BorrowMut;
use grimstone::world::Block::Air;

fn main() -> Result<(), Error> {
    Server::global().world = Some(World::new(Box::new(SingleWorldFile::new("world.sng"))));
    let w = Server::global().world.as_mut().unwrap();
    w.load_chunk(0, 0, 0);

    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();
    log::debug!("Initializing server");

    let config = &ConcreteConfig::from(Config::load());
    let server = Arc::new(TcpListener::bind(
        format!("127.0.0.1:{}", config.server_port))
        .expect("Could not create server"));

    log::info!("Server started on port {}", config.server_port);

    loop {
        let result = server.accept();
        if let Ok((stream, addr)) = result {
            let conf = config.clone();
            thread::spawn(move || {
                let mut client = Client::new(stream, conf);
                packets::register(&mut client);
                while client.is_valid {
                    let mut packet_result = client.read_packet();
                    match packet_result {
                        Ok(packet) => {
                            packet.act(&mut client).expect("Error while processing packet {}");
                        }
                        Err(error) => {
                            match error {
                                Error::Refusal => {
                                    log::warn!("Something refused to execute; check your wiring!");
                                },
                                Error::Disconnected => {
                                    log::info!("Client {} disconnected", addr);
                                    break;
                                },
                                _ => {
                                    log::error!("Encountered an error: {:?}", error);
                                    break;
                                }
                            }
                        }
                    }
                }
            });
        }
    }

    Ok(())
}
