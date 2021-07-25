use serde::{Serialize, Deserialize};
use nbt::{Tag, Blob};

#[derive(Serialize)]
pub struct Registry<'a, T: Serialize> {
    #[serde(rename = "type")]
    pub name: &'a str,
    #[serde(rename = "value")]
    pub entries: Vec<RegistryEntry<T>>
}

#[derive(Serialize)]
pub struct RegistryEntry<T: Serialize> {
    pub name: String,
    pub id: i32,
    pub element: T
}

impl<'a, T: Serialize> Registry<'a, T> {
    pub fn encode(&mut self) -> Blob {
        nbt::encode(self).expect("Failed to encode registry")
    }

    pub fn register(&mut self, name: &str, value: T) {
        self.entries.sort_by(|a, b| a.id.cmp(&b.id));
        let mut id = 0;

        for e in &self.entries {
            if e.id == id { id += 1; }
            else { break; }
        }

        self.entries.push(RegistryEntry::<T> { name: name.to_string(), id, element: value})
    }
}
