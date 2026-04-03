use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_included_mod_dirs")]
    pub included_mod_dirs: Vec<ModDirConfig>,
    #[serde(default)]
    pub game_executable_path: String,
    #[serde(default)]
    pub world_seed: Option<i64>,
    #[serde(default)]
    pub reset_world: bool,
    #[serde(default = "default_world_name")]
    pub world_name: String,
    #[serde(default = "default_world_folder_name")]
    pub world_folder_name: String,
    #[serde(default = "default_true")]
    pub auto_join_game: bool,
    #[serde(default = "default_true")]
    pub include_debug_mod: bool,
    #[serde(default = "default_true")]
    pub auto_hot_reload_mods: bool,
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
    pub experiment_options: ExperimentOptions,
    #[serde(default = "default_user_name")]
    pub user_name: String,
    #[serde(default)]
    pub skin_info: Option<SkinInfo>,
    #[serde(default)]
    pub modpc_debugger: Option<ModpcDebuggerConfig>,
    #[serde(default)]
    pub debug_options: Option<DebugOptions>,
    #[serde(default)]
    pub window_style: Option<WindowStyle>,
    #[serde(default)]
    pub netease_config: Option<NeteaseConfig>,
}

fn default_included_mod_dirs() -> Vec<ModDirConfig> {
    vec![ModDirConfig {
        path: "./".to_string(),
        hot_reload: true,
        enabled: true,
    }]
}

fn default_world_name() -> String {
    "MC_DEV_WORLD".to_string()
}

fn default_world_folder_name() -> String {
    "MC_DEV_WORLD".to_string()
}

fn default_true() -> bool {
    true
}

fn default_world_type() -> u32 {
    1
}

fn default_game_mode() -> u32 {
    1
}

fn default_user_name() -> String {
    "developer".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModDirConfig {
    pub path: String,
    #[serde(default = "default_true")]
    pub hot_reload: bool,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExperimentOptions {
    #[serde(default)]
    pub data_driven_biomes: bool,
    #[serde(default)]
    pub data_driven_items: bool,
    #[serde(default)]
    pub experimental_molang_features: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinInfo {
    #[serde(default)]
    pub slim: bool,
    pub skin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModpcDebuggerConfig {
    #[serde(default = "default_false")]
    pub enabled: bool,
    #[serde(default = "default_port")]
    pub port: u16,
}

fn default_false() -> bool {
    false
}

fn default_port() -> u16 {
    5632
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DebugOptions {
    #[serde(default)]
    pub reload_key: String,
    #[serde(default)]
    pub reload_world_key: String,
    #[serde(default)]
    pub reload_addon_key: String,
    #[serde(default)]
    pub reload_shaders_key: String,
    #[serde(default = "default_false")]
    pub reload_key_global: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WindowStyle {
    #[serde(default)]
    pub always_on_top: bool,
    #[serde(default)]
    pub hide_title_bar: bool,
    #[serde(default)]
    pub title_bar_color: Option<Vec<u8>>,
    #[serde(default)]
    pub fixed_size: Option<Vec<u32>>,
    #[serde(default)]
    pub fixed_position: Option<Vec<i32>>,
    #[serde(default)]
    pub lock_corner: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NeteaseConfig {
    #[serde(default)]
    pub chat_extension: bool,
}

impl Config {
    pub fn load(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}