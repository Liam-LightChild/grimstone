#![cfg(feature = "single")]

use crate::world::{WorldSyncer, World, Block, ChunkLoadState, ChunkContainer, Chunk};
use std::path::Path;
use std::collections::HashMap;
use crate::Vector3I;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write, Read};
use SeekFrom::{Start, Current};
use byteorder::{WriteBytesExt, ReadBytesExt, BigEndian};
use std::mem::{size_of, transmute};
use crate::world::ChunkLoadState::{Loaded, Unloaded};
use std::io::SeekFrom::End;

pub struct SingleWorldFile {
    file: File,
    indices: HashMap<Vector3I, usize>,
    current_chunk_count: u64
}

impl WorldSyncer for SingleWorldFile {
    fn new(path: &str) -> Self where Self: Sized {
        let mut file = if Path::new(path).exists() {
            OpenOptions::new()
                .read(true)
                .write(true)
                .open(path)
                .expect("Could not open existing file?")
        } else {
            {
                let mut file = File::create(path).expect("Could not create file.");
                file.write_all("SNG\0".as_bytes()).expect("Could not write data to Single file");
                file.write_u64::<BigEndian>(0).expect("Could not write data to Single file");
                file.seek(Start(0)).expect("Could not seek into Single file");
            }
            OpenOptions::new()
                .read(true)
                .write(true)
                .open(path)
                .expect("Could not open new file.")
        };

        {
            let mut header_array = [0u8; 4];
            file.read(&mut header_array).expect("Could not read data from Single file; may be invalid");
            if String::from_utf8(header_array.to_vec()).expect("UTF-8 is not cooperating") != "SNG\0".to_string() {
                panic!("Invalid Single world {}", path);
            }
        }

        let mut indices = HashMap::new();
        let index_count = file.read_u64::<BigEndian>().expect("Could not read data from single file; may be invalid");

        // Single World Chunk format:
        // - <x>: i32
        // - <y>: i32
        // - <z>: i32
        // - <blocks>: [Block; 16*16*16]
        for _ in 0..index_count {
            let start_index = file.stream_position().expect("Could not get position of stream.");
            let x = file.read_i32::<BigEndian>().expect("Could not read data from single file; may be invalid");
            let y = file.read_i32::<BigEndian>().expect("Could not read data from single file; may be invalid");
            let z = file.read_i32::<BigEndian>().expect("Could not read data from single file; may be invalid");
            indices.insert(Vector3I(x as i64, y as i64, z as i64), start_index as usize);
            file.seek(Current(size_of::<[Block; 16 * 16 * 16]>() as i64)).expect("Could not seek into Single file");
        }

        Self { file, indices, current_chunk_count: index_count }
    }

    fn save(&mut self, chunk: &mut Chunk) {
        let pos = Vector3I(chunk.x as i64, chunk.y as i64, chunk.z as i64);
        if self.indices.contains_key(&pos) {
            self.file.seek(Start(*self.indices.get(&pos).unwrap() as u64)).expect("Could not seek into Single file");
        } else {
            self.file.seek(End(0)).expect("Could not seek into Single file");
            self.indices.insert(pos, self.file.stream_position().expect("Could not get position of stream.") as usize);
            self.current_chunk_count += 1;
        }
        self.file.write_i32::<BigEndian>(chunk.x).expect("Could not write data to Single file");
        self.file.write_i32::<BigEndian>(chunk.y).expect("Could not write data to Single file");
        self.file.write_i32::<BigEndian>(chunk.z).expect("Could not write data to Single file");
        for b in chunk.blocks {
            self.file.write_u16::<BigEndian>(b as u16).expect("Could not write data to Single file");
        }
        self.file.seek(Start(4)).expect("Could not seek into Single file");
        self.file.write_u64::<BigEndian>(self.current_chunk_count).expect("Could not write data to Single file");
        self.file.flush().expect("Could not flush Single file");
    }

    fn find_all(&mut self) -> Vec<Vector3I> {
        let mut poses = vec![];
        for p in self.indices.keys() { poses.push(*p); }
        poses
    }

    fn load(&mut self, x: i32, y: i32, z: i32) -> Chunk {
        let pos = Vector3I(x as i64, y as i64, z as i64);
        if self.indices.contains_key(&pos) {
            let mut chunk = Chunk::new_empty(x, y, z);
            self.file.seek(Start((self.indices.get(&pos).unwrap() + 12) as u64)).expect("Could not seek into Single file");
            for i in 0..(16 * 16 * 16) {
                chunk.blocks[i] = unsafe { transmute(self.file.read_u16::<BigEndian>().expect("Could not read data from single file; may be invalid")) };
            }
            chunk
        } else {
            Chunk::generate_new(x, y, z)
        }
    }
}