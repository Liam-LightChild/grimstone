use crate::GameMode;
use crate::traits::{Packet, Readable, Writable};
use crate::client::{Error, Client};
use crate::client::Error::Refusal;
use serde::Serialize;
use nbt::{Blob, Tag, NBTWrite};
use std::collections::HashMap;
use crate::registry::Registry;

#[derive(Debug, Copy, Clone)]
pub struct JoinGamePacket {
    pub eid: i32,
    pub game_mode: GameMode
}

#[derive(Serialize)]
pub struct DimType<'str> {
    pub piglin_safe: bool,
    pub natural: bool,
    pub ambient_light: f32,
    pub infiniburn: &'str str,
    pub respawn_anchor_works: bool,
    pub has_skylight: bool,
    pub bed_works: bool,
    pub effects: &'str str,
    pub has_raids: bool,
    pub min_y: i32,
    pub height: i32,
    pub logical_height: i32,
    pub coordinate_scale: f64,
    pub ultrawarm: bool,
    pub has_ceiling: bool
}

#[derive(Serialize)]
pub struct BiomeMoodSound<'str> {
    pub sound: &'str str,
    pub tick_delay: i32,
    pub offset: f64,
    pub block_search_extent: i32
}

#[derive(Serialize)]
pub struct BiomeAdditionsSound<'str> {
    pub sound: &'str str,
    pub tick_chance: f64
}

#[derive(Serialize)]
pub struct BiomeMusic<'str> {
    pub replace_current_music: bool,
    pub sound: &'str str,
    pub max_delay: i32,
    pub min_delay: i32
}

#[derive(Serialize)]
pub struct BiomeParticleOptions<'str> {
    #[serde(rename = "type")]
    pub particle_type: &'str str
}

#[derive(Serialize)]
pub struct BiomeParticles<'str> {
    pub probability: f32,
    pub options: BiomeParticleOptions<'str>
}

#[derive(Serialize)]
pub struct BiomeEffects<'str> {
    pub sky_color: i32,
    pub water_fog_color: i32,
    pub fog_color: i32,
    pub water_color: i32,
    pub foliage_color: Option<i32>,
    pub grass_color: Option<i32>,
    pub grass_color_modifier: Option<&'str str>,
    pub music: Option<BiomeMusic<'str>>,
    pub ambient_sound: Option<&'str str>,
    pub additions_sound: Option<BiomeAdditionsSound<'str>>,
    pub mood_sound: Option<BiomeMoodSound<'str>>,
}

#[derive(Serialize)]
pub struct Biome<'str> {
    pub precipitation: &'str str,
    pub depth: f32,
    pub temperature: f32,
    pub scale: f32,
    pub downfall: f32,
    pub category: &'str str,
    pub temperature_modifier: Option<&'str str>,
    pub particle: Option<BiomeParticles<'str>>,
    pub effects: BiomeEffects<'str>
}

impl<'a> DimType<'a> {
    pub const OVERWORLD: DimType<'a> = DimType::<'a> {
        piglin_safe: false,
        natural: true,
        ambient_light: 0.0,
        infiniburn: "minecraft:infiniburn_overworld",
        respawn_anchor_works: false,
        has_skylight: true,
        bed_works: true,
        effects: "minecraft:overworld",
        has_raids: true,
        min_y: 0,
        height: 256,
        logical_height: 256,
        coordinate_scale: 1.0,
        ultrawarm: false,
        has_ceiling: false
    };
}

impl<'str> Biome<'str> {
    pub const OCEAN: Biome<'str> = Biome::<'str> {
        precipitation: "rain",
        depth: -1.0,
        temperature: 0.5,
        scale: 0.1,
        downfall: 0.5,
        category: "ocean",
        temperature_modifier: None,
        particle: None,
        effects: BiomeEffects {
            sky_color: 8103167,
            water_fog_color: 329011,
            fog_color: 12638463,
            water_color: 4159204,
            foliage_color: None,
            grass_color: None,
            grass_color_modifier: None,
            music: None,
            ambient_sound: None,
            additions_sound: None,
            mood_sound: Some(BiomeMoodSound {
                sound: "minecraft:ambient.cave",
                tick_delay: 6000,
                offset: 2.0,
                block_search_extent: 8
            })
        }
    };
}

impl Packet for JoinGamePacket {
    fn id(&self) -> u32 { 0x26 }

    fn read(_: &mut dyn Readable) -> Result<Self, Error> where Self: Sized {
        Err(Refusal)
    }

    fn write(&self, output: &mut dyn Writable) -> Result<(), Error> {
        output.write_i32(self.eid)?;
        output.write_u8(0)?;
        output.write_u8(match self.game_mode {
            GameMode::Survival => 0
        })?;
        output.write_i8(-1)?;
        output.write_var_int(1)?;
        output.write_string("minecraft:overworld".to_string())?;

        let mut dim_types = Registry::<DimType> {
            name: "minecraft:dimension_type",
            entries: vec![]
        };

        dim_types.register("minecraft:overworld", DimType::OVERWORLD);

        let mut biomes = Registry::<Biome> {
            name: "minecraft:worldgen/biome",
            entries: vec![]
        };

        biomes.register("minecraft:ocean", Biome::OCEAN);

        let mut blob = Blob::new();
        blob.insert("minecraft:dimension_type", dim_types.encode().compound());
        blob.insert("minecraft:worldgen/biome", biomes.encode().compound());
        let bytes_vec = blob.bytes().unwrap();
        println!("{:#?}", blob);

        output.write(bytes_vec.as_slice())?;
        std::fs::write("debug.nbt", bytes_vec.as_slice());
        output.write(nbt::encode(&DimType::OVERWORLD).unwrap().bytes().unwrap().as_slice())?;

        output.write_string("minecraft:overworld".to_string())?;
        output.write_u64(0)?;
        output.write_var_int(0)?;
        output.write_var_int(4)?;
        output.write_u8(0)?;
        output.write_u8(1)?;
        output.write_u8(0)?;
        output.write_u8(1)?;

        Ok(())
    }

    fn act(&self, client: &mut Client) -> Result<(), Error> {
        Ok(())
    }
}
