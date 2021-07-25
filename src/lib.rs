use std::fmt::{Display, Formatter, Write};

pub mod client;
pub mod config;
pub mod packets;
pub mod traits;
pub mod buffer;
pub mod world;
pub mod server;
#[cfg(feature = "single")] pub mod single;
mod registry;

pub const MINECRAFT_VERSION: &str = "1.17.0";
pub const GRIMSTONE_VERSION: &str = "1.17.0.2";
pub const MINECRAFT_PROTOCOL_VERSION: u32 = 755;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct NamespacedId<'id>(&'id str, &'id str);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Vector3I(i64, i64, i64);

impl<'a> Display for NamespacedId<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)?;
        f.write_char(':')?;
        f.write_str(self.1)?;
        Ok(())
    }
}

#[inline]
pub const fn nsid<'a>(ns: &'a str, id: &'a str) -> NamespacedId<'a> {
    NamespacedId(ns, id)
}
