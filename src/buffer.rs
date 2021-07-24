use crate::traits::{Readable, Writable};
use crate::client::Error;
use crate::client::Error::Disconnected;

pub struct Buffer {
    pub bytes: Vec<u8>
}

impl Buffer {
    pub(crate) fn new() -> Self {
        Self {
            bytes: vec![]
        }
    }
}

impl From<&[u8]> for Buffer {
    fn from(b: &[u8]) -> Self {
        Self {
            bytes: Vec::from(b)
        }
    }
}

impl Readable for Buffer {
    fn read(&mut self, array: &mut [u8]) -> Result<usize, Error> {
        if array.len() > self.bytes.len() { return Err(Disconnected); }
        array.copy_from_slice(&self.bytes[..array.len()]);
        let new_start = self.bytes.len() - array.len();
        let new_data = &self.bytes[array.len()..].to_vec();
        self.bytes[..new_start].copy_from_slice(new_data.as_slice());
        Ok(array.len())
    }
}

impl Writable for Buffer {
    fn write(&mut self, array: &[u8]) -> Result<usize, Error> {
        self.bytes.append(&mut array.to_vec());
        Ok(array.len())
    }
}
