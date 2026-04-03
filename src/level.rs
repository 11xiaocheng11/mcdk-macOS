use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExperimentsOptions {
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    pub data_driven_biomes: bool,
    #[serde(default)]
    pub data_driven_items: bool,
    #[serde(default)]
    pub experimental_molang_features: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelOptions {
    #[serde(default)]
    pub seed: Option<i64>,
    #[serde(default = "default_world_type")]
    pub world_type: u32,
    #[serde(default = "default_game_mode")]
    pub game_mode: u32,
    #[serde(default = "default_true")]
    pub enable_cheats: bool,
    #[serde(default = "default_true")]
    pub keep_inventory: bool,
    #[serde(default = "default_true")]
    pub do_weather_cycle: bool,
    #[serde(default = "default_true")]
    pub do_daylight_cycle: bool,
    #[serde(default)]
    pub experiments_options: ExperimentsOptions,
}

fn default_world_type() -> u32 { 1 }
fn default_game_mode() -> u32 { 1 }
fn default_true() -> bool { true }

pub fn create_default_level_dat(world_name: &str, options: &LevelOptions) -> Vec<u8> {
    let mut template = get_level_dat_template();
    update_level_dat_world_data(&mut template, Some(world_name), options, true);
    template
}

#[cfg(feature = "nbt")]
pub fn update_level_dat_world_data(
    level_dat_data: &mut Vec<u8>,
    world_name: Option<&str>,
    options: &LevelOptions,
    init: bool,
) {
    use std::io::{Cursor, Read};
    let mut cursor = Cursor::new(level_dat_data);
    let mut bytes = Vec::new();
    cursor.read_to_end(&mut bytes).unwrap();
    
    let content = String::from_utf8_lossy(&bytes);
    
    match content.parse::<nbt::Nbt>() {
        Ok(mut tag) => {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            
            tag.insert("LastPlayed".to_string(), nbt::Nbt::Long(now));
            
            if let Some(name) = world_name {
                tag.insert("LevelName".to_string(), nbt::Nbt::String(name.to_string()));
            }
            
            if options.world_type != 2 && init {
                let seed = options.seed.unwrap_or_else(|| {
                    use std::collections::hash_map::RandomState;
                    use std::hash::{BuildHasher, Hasher};
                    let s = RandomState::new().build_hasher();
                    s.finish() as i64
                });
                tag.insert("RandomSeed".to_string(), nbt::Nbt::Long(seed));
            }
            
            tag.insert("GameType".to_string(), nbt::Nbt::Int(options.game_mode as i32));
            
            if init {
                tag.insert("Generator".to_string(), nbt::Nbt::Int(options.world_type as i32));
            }
            
            tag.insert("keepInventory".to_string(), nbt::Nbt::Byte(if options.keep_inventory { 1 } else { 0 }));
            tag.insert("cheatsEnabled".to_string(), nbt::Nbt::Byte(if options.enable_cheats { 1 } else { 0 }));
            tag.insert("doweathercycle".to_string(), nbt::Nbt::Byte(if options.do_weather_cycle { 1 } else { 0 }));
            tag.insert("dodaylightcycle".to_string(), nbt::Nbt::Byte(if options.do_daylight_cycle { 1 } else { 0 }));
            
            if options.experiments_options.enable {
                let mut experiments = nbt::Nbt::Compound(std::collections::HashMap::new());
                experiments.insert("data_driven_biomes".to_string(), nbt::Nbt::Byte(if options.experiments_options.data_driven_biomes { 1 } else { 0 }));
                experiments.insert("data_driven_items".to_string(), nbt::Nbt::Byte(if options.experiments_options.data_driven_items { 1 } else { 0 }));
                experiments.insert("experimental_molang_features".to_string(), nbt::Nbt::Byte(if options.experiments_options.experimental_molang_features { 1 } else { 0 }));
                tag.insert("experiments".to_string(), experiments);
            }
            
            let output = tag.to_writer(Vec::new()).unwrap();
            level_dat_data.clear();
            level_dat_data.extend(output);
        }
        Err(e) => {
            eprintln!("Failed to parse NBT: {}", e);
        }
    }
}

#[cfg(not(feature = "nbt"))]
pub fn update_level_dat_world_data(
    level_dat_data: &mut Vec<u8>,
    world_name: Option<&str>,
    _options: &LevelOptions,
    _init: bool,
) {
    if let Some(name) = world_name {
        eprintln!("Note: NBT support not enabled, world name '{}' not applied", name);
    }
    eprintln!("Note: NBT support not enabled, using raw template");
}

#[cfg(feature = "nbt")]
pub fn update_level_dat_last_played(level_dat_data: &mut Vec<u8>) {
    let content = String::from_utf8_lossy(level_dat_data);
    match content.parse::<nbt::Nbt>() {
        Ok(mut tag) => {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            tag.insert("LastPlayed".to_string(), nbt::Nbt::Long(now));
            
            let output = tag.to_writer(Vec::new()).unwrap();
            level_dat_data.clear();
            level_dat_data.extend(output);
        }
        Err(e) => {
            eprintln!("Failed to parse NBT: {}", e);
        }
    }
}

#[cfg(not(feature = "nbt"))]
pub fn update_level_dat_last_played(_level_dat_data: &mut Vec<u8>) {
    eprintln!("Note: NBT support not enabled");
}

pub fn update_level_dat_last_played_in_file(file_path: &std::path::Path) -> std::io::Result<()> {
    let mut data = std::fs::read(file_path)?;
    update_level_dat_last_played(&mut data);
    std::fs::write(file_path, data)?;
    Ok(())
}

pub fn update_level_dat_world_data_in_file(
    file_path: &std::path::Path,
    world_name: Option<&str>,
    options: &LevelOptions,
) -> std::io::Result<()> {
    let mut data = std::fs::read(file_path)?;
    update_level_dat_world_data(&mut data, world_name, options, false);
    std::fs::write(file_path, data)?;
    Ok(())
}

fn get_level_dat_template() -> Vec<u8> {
    include_bytes!("../../../tests/bins/level.dat").to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_dat_creation() {
        let options = LevelOptions {
            seed: Some(12345),
            world_type: 1,
            game_mode: 1,
            ..Default::default()
        };
        
        let data = create_default_level_dat("TestWorld", &options);
        assert!(!data.is_empty());
    }
}