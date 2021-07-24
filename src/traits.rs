use crate::client::{Client, Error};
use std::mem::{transmute, size_of};

pub trait Packet {
    fn id(&self) -> u32;
    fn read(input: &mut dyn Readable) -> Result<Self, Error> where Self : Sized;
    fn write(&self, output: &mut dyn Writable) -> Result<(), Error>;
    fn act(&self, client: &mut Client) -> Result<(), Error>;
}

pub trait Writable {
    fn write(&mut self, array: &[u8]) -> Result<usize, Error>;

    fn write_i8(&mut self, value: i8) -> Result<usize, Error> {
        unsafe {
            let array = &mut transmute::<i8, [u8; size_of::<i8>()]>(value.to_be());
            Ok(self.write(array)?)
        }
    }

    fn write_i16(&mut self, value: i16) -> Result<usize, Error> {
        unsafe {
            let array = &mut transmute::<i16, [u8; size_of::<i16>()]>(value.to_be());
            if cfg!(target_endian = "little") { array.reverse() }
            Ok(self.write(array)?)
        }
    }

    fn write_i32(&mut self, value: i32) -> Result<usize, Error> {
        unsafe {
            let array = &mut transmute::<i32, [u8; size_of::<i32>()]>(value.to_be());
            if cfg!(target_endian = "little") { array.reverse() }
            Ok(self.write(array)?)
        }
    }

    fn write_i64(&mut self, value: i64) -> Result<usize, Error> {
        unsafe {
            let array = &mut transmute::<i64, [u8; size_of::<i64>()]>(value.to_be());
            if cfg!(target_endian = "little") { array.reverse() }
            Ok(self.write(array)?)
        }
    }

    fn write_i128(&mut self, value: i128) -> Result<usize, Error> {
        unsafe {
            let array = &mut transmute::<i128, [u8; size_of::<i128>()]>(value.to_be());
            if cfg!(target_endian = "little") { array.reverse() }
            Ok(self.write(array)?)
        }
    }

    fn write_u8(&mut self, value: u8) -> Result<usize, Error> {
        unsafe {
            let array = &mut transmute::<u8, [u8; size_of::<u8>()]>(value);
            Ok(self.write(array)?)
        }
    }

    fn write_u16(&mut self, value: u16) -> Result<usize, Error> {
        unsafe {
            let array = &mut transmute::<u16, [u8; size_of::<u16>()]>(value.to_be());
            if cfg!(target_endian = "little") { array.reverse() }
            Ok(self.write(array)?)
        }
    }

    fn write_u32(&mut self, value: u32) -> Result<usize, Error> {
        unsafe {
            let array = &mut transmute::<u32, [u8; size_of::<u32>()]>(value.to_be());
            if cfg!(target_endian = "little") { array.reverse() }
            Ok(self.write(array)?)
        }
    }

    fn write_u64(&mut self, value: u64) -> Result<usize, Error> {
        unsafe {
            let array = &mut transmute::<u64, [u8; size_of::<u64>()]>(value.to_be());
            if cfg!(target_endian = "little") { array.reverse() }
            Ok(self.write(array)?)
        }
    }

    fn write_u128(&mut self, value: u128) -> Result<usize, Error> {
        unsafe {
            let array = &mut transmute::<u128, [u8; size_of::<u128>()]>(value.to_be());
            if cfg!(target_endian = "little") { array.reverse() }
            Ok(self.write(array)?)
        }
    }

    fn write_usize(&mut self, value: usize) -> Result<usize, Error> {
        unsafe {
            let array = &mut transmute::<usize, [u8; size_of::<usize>()]>(value.to_be());
            if cfg!(target_endian = "little") { array.reverse() }
            Ok(self.write(array)?)
        }
    }

    fn write_isize(&mut self, value: isize) -> Result<usize, Error> {
        unsafe {
            let array = &mut transmute::<isize, [u8; size_of::<isize>()]>(value.to_be());
            if cfg!(target_endian = "little") { array.reverse() }
            Ok(self.write(array)?)
        }
    }

    fn write_var_int(&mut self, mut value: i32) -> Result<usize, Error> {
        let mut count = 0usize;
        loop {
            let mut u = (value & 0b01111111i32) as u8;
            count += 1;
            value &= !0b01111111i32;
            value >>= 7;
            if value != 0 { u |= 0b10000000; }
            self.write_u8(u)?;
            if value == 0 { break; }
        }
        Ok(count)
    }

    fn write_var_long(&mut self, mut value: i64) -> Result<usize, Error> {
        let mut count = 0usize;
        loop {
            let u = (value & 0b01111111i64) as u8;
            self.write_u8(u)?;
            count += 1;
            value &= !0b01111111i64;
            value >>= 7;
            if value == 0 { break; }
        }
        Ok(count)
    }

    fn write_string(&mut self, value: String) -> Result<usize, Error> {
        let mut size = self.write_var_int(value.len() as i32)?;
        size += self.write(value.as_bytes())?;
        Ok(size)
    }
}

pub trait Readable {
    fn read(&mut self, array: &mut [u8]) -> Result<usize, Error>;

    fn read_i8(&mut self) -> Result<i8, Error> {
        let mut a = [0u8; std::mem::size_of::<i8>()];
        self.read(a.as_mut())?;
        unsafe { Ok(transmute(a)) }
    }

    fn read_i16(&mut self) -> Result<i16, Error> {
        let mut a = [0u8; std::mem::size_of::<i16>()];
        self.read(a.as_mut())?;
        if cfg!(target_endian = "little") { a.reverse(); } // endianness
        unsafe { Ok(transmute(a)) }
    }

    fn read_i32(&mut self) -> Result<i32, Error> {
        let mut a = [0u8; std::mem::size_of::<i32>()];
        self.read(a.as_mut())?;
        if cfg!(target_endian = "little") { a.reverse(); } // endianness
        unsafe { Ok(transmute(a)) }
    }

    fn read_i64(&mut self) -> Result<i64, Error> {
        let mut a = [0u8; std::mem::size_of::<i64>()];
        self.read(a.as_mut())?;
        if cfg!(target_endian = "little") { a.reverse(); } // endianness
        unsafe { Ok(transmute(a)) }
    }

    fn read_i128(&mut self) -> Result<i128, Error> {
        let mut a = [0u8; std::mem::size_of::<i128>()];
        self.read(a.as_mut())?;
        if cfg!(target_endian = "little") { a.reverse(); } // endianness
        unsafe { Ok(transmute(a)) }
    }

    fn read_u8(&mut self) -> Result<u8, Error> {
        let mut a = [0u8; std::mem::size_of::<u8>()];
        self.read(a.as_mut())?;
        unsafe { Ok(transmute(a)) }
    }

    fn read_u16(&mut self) -> Result<u16, Error> {
        let mut a = [0u8; std::mem::size_of::<u16>()];
        self.read(a.as_mut())?;
        if cfg!(target_endian = "little") { a.reverse(); } // endianness
        unsafe { Ok(transmute(a)) }
    }

    fn read_u32(&mut self) -> Result<u32, Error> {
        let mut a = [0u8; std::mem::size_of::<u32>()];
        self.read(a.as_mut())?;
        if cfg!(target_endian = "little") { a.reverse(); } // endianness
        unsafe { Ok(transmute(a)) }
    }

    fn read_u64(&mut self) -> Result<u64, Error> {
        let mut a = [0u8; std::mem::size_of::<u64>()];
        self.read(a.as_mut())?;
        if cfg!(target_endian = "little") { a.reverse(); } // endianness
        unsafe { Ok(transmute(a)) }
    }

    fn read_u128(&mut self) -> Result<u128, Error> {
        let mut a = [0u8; std::mem::size_of::<u128>()];
        self.read(a.as_mut())?;
        if cfg!(target_endian = "little") { a.reverse(); } // endianness
        unsafe { Ok(transmute(a)) }
    }

    fn read_usize(&mut self) -> Result<usize, Error> {
        let mut a = [0u8; std::mem::size_of::<usize>()];
        self.read(a.as_mut())?;
        if cfg!(target_endian = "little") { a.reverse(); } // endianness
        unsafe { Ok(transmute(a)) }
    }

    fn read_isize(&mut self) -> Result<isize, Error> {
        let mut a = [0u8; std::mem::size_of::<isize>()];
        self.read(a.as_mut())?;
        if cfg!(target_endian = "little") { a.reverse(); } // endianness
        unsafe { Ok(transmute(a)) }
    }

    fn read_var_int(&mut self) -> Result<i32, Error> {
        let mut i = 0i32;
        loop {
            let u = self.read_u8()?;
            i <<= 7;
            i |= (u & !0b10000000) as i32;
            if u & 0b10000000 == 0 { break; }
        }
        Ok(i)
    }

    fn read_var_long(&mut self) -> Result<i64, Error> {
        let mut i = 0i64;
        loop {
            let u = self.read_u8()?;
            i <<= 7;
            i |= (u & !0b10000000) as i64;
            if u & 0b10000000 == 0 { break; }
        }
        Ok(i)
    }

    fn read_string(&mut self, max_size: usize) -> Result<String, Error> {
        let len = self.read_var_int()? as usize;
        let mut chars = vec![0u8; len as usize];
        self.read(chars.as_mut_slice())?;
        if len >= max_size {
            Err(Error::StringTooLong(len as usize, max_size, String::from_utf8(chars).unwrap()))
        } else {
            Ok(String::from_utf8(chars).unwrap())
        }
    }
}
