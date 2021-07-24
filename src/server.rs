use crate::client::Client;
use crate::world::World;
#[cfg(feature = "world_syncers")] use crate::world::WorldSyncer;

pub struct Server {
    pub clients: Vec<Client>,
    pub world: Option<World>
}

static mut SERVER: Server = Server {
    clients: vec![],
    world: None
};

impl Server {
    pub fn global() -> &'static mut Server {
        unsafe { &mut SERVER }
    }
}