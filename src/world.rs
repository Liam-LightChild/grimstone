use crate::Vector3I;
use std::collections::HashMap;
use ChunkLoadState::Loaded;
use crate::world::ChunkLoadState::Unloaded;
use std::ops::{DerefMut, Deref};

#[repr(u16)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Block {
    Air
}

pub trait BlockContainer {
    fn get(&mut self, x: i64, y: i64, z: i64) -> Block;
    fn put(&mut self, x: i64, y: i64, z: i64, block: Block);
}

pub trait ChunkContainer {
    //noinspection RsSelfConvention
    fn get_chunk(&mut self, x: i32, y: i32, z: i32) -> &mut Chunk;
    fn put_chunk(&mut self, x: i32, y: i32, z: i32, chunk: Chunk);
    fn get(&mut self, x: i64, y: i64, z: i64) -> Block;
    fn put(&mut self, x: i64, y: i64, z: i64, block: Block);
}

#[cfg(feature = "world_syncers")]
pub trait WorldSyncer {
    fn new(path: &str) -> Self where Self: Sized;
    fn save(&mut self, chunk: &mut Chunk);
    fn find_all(&mut self) -> Vec<Vector3I>;
    fn load(&mut self, x: i32, y: i32, z: i32) -> Chunk;
}

pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub(crate) blocks: [Block; 16*16*16]
}

impl Chunk {
    pub fn generate_new(x: i32, y: i32, z: i32) -> Chunk {
        Chunk::new_empty(x, y, z)
    }
}

pub(crate) enum ChunkLoadState {
    Unloaded,
    Loaded { chunk: Chunk }
}

pub struct World {
    pub(crate) chunks: HashMap<Vector3I, ChunkLoadState>,
    pub sync: Box<dyn WorldSyncer>
}

impl Chunk {
    pub fn new_empty(x: i32, y: i32, z: i32) -> Chunk {
        Chunk {
            x, y, z,
            blocks: [Block::Air; 16*16*16]
        }
    }

    pub fn new_from_block(x: i32, y: i32, z: i32, block: Block) -> Chunk {
        Chunk {
            x, y, z,
            blocks: [block; 16*16*16]
        }
    }
}

impl BlockContainer for Chunk {
    fn get(&mut self, x: i64, y: i64, z: i64) -> Block {
        if x < 0 || x >= 16 || y < 0 || y >= 16 || z < 0 || z >= 16 {
            panic!("Invalid position in chunk [{},{},{}]", x, y, z);
        }

        let mut i = 0usize;
        i |= y as usize;
        i <<= 4;
        i |= z as usize;
        i <<= 4;
        i |= x as usize;

        #[cfg(feature = "debug")]
        log::debug!("{},{},{} = idx {}; is {:?}", x, y, z, i, self.blocks[i]);

        self.blocks[i]
    }

    fn put(&mut self, x: i64, y: i64, z: i64, block: Block) {
        if x < 0 || x >= 16 || y < 0 || y >= 16 || z < 0 || z >= 16 {
            panic!("Invalid position in chunk [{},{},{}]", x, y, z);
        }

        let mut i = 0usize;
        i |= y as usize;
        i <<= 4;
        i |= z as usize;
        i <<= 4;
        i |= x as usize;

        #[cfg(feature = "debug")]
        log::debug!("{},{},{} = idx {}; now {:?}", x, y, z, i, block);

        self.blocks[i] = block;
    }
}

impl World {
    pub fn new(mut sync: Box<dyn WorldSyncer>) -> World {
        let mut chunks = HashMap::new();
        for p in sync.find_all() {
            chunks.insert(p, Unloaded);
        }
        World { chunks, sync }
    }

    pub fn load_chunk(&mut self, x: i32, y: i32, z: i32) -> &mut Chunk {
        let pos = Vector3I(x as i64, y as i64, z as i64);
        if !self.chunks.contains_key(&pos) {
            let chunk = self.sync.load(x, y, z);
            self.chunks.insert(pos, Loaded { chunk });
        } else if let Unloaded = self.chunks.get(&pos).unwrap() {
            let chunk = self.sync.load(x, y, z);
            self.chunks.insert(pos, Loaded { chunk });
        }
        self.get_chunk(x, y, z)
    }

    pub fn unload_chunk(&mut self, x: i32, y: i32, z: i32) {
        let pos = Vector3I(x as i64, y as i64, z as i64);
        if self.chunks.contains_key(&pos) {
            self.chunks.insert(pos, Unloaded);
        }
    }
}

impl ChunkContainer for World {
    fn get_chunk(&mut self, x: i32, y: i32, z: i32) -> &mut Chunk {
        let pos = Vector3I(x as i64, y as i64, z as i64);

        if self.chunks.contains_key(&pos) {
            let load_state = self.chunks.get_mut(&pos).unwrap();

            if let Loaded { chunk, .. } = load_state {
                chunk
            } else {
                panic!()
            }
        } else {
            let chunk = Chunk::new_empty(x, y, z);
            self.chunks.insert(pos, Loaded { chunk });
            if let Loaded { chunk, .. } = self.chunks.get_mut(&pos).unwrap() {
                chunk
            } else {
                panic!("Weird state for chunk.")
            }
        }
    }

    fn put_chunk(&mut self, x: i32, y: i32, z: i32, chunk: Chunk) {
        let pos = Vector3I(x as i64, y as i64, z as i64);

        self.chunks.insert(pos, Loaded { chunk });
    }

    fn get(&mut self, x: i64, y: i64, z: i64) -> Block {
        todo!()
    }

    fn put(&mut self, x: i64, y: i64, z: i64, block: Block) {
        todo!()
    }
}
