use std::path::{Path, PathBuf};

pub fn get_app_data_path() -> PathBuf {
    #[cfg(windows)]
    {
        std::env::var("APPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("C:/Users/default/AppData/Roaming"))
    }
    #[cfg(target_os = "macos")]
    {
        std::env::var("HOME")
            .map(|h| PathBuf::from(h).join("Library/Application Support"))
            .unwrap_or_else(|_| PathBuf::from("/Users/default/Library/Application Support"))
    }
    #[cfg(target_os = "linux")]
    {
        std::env::var("HOME")
            .map(|h| PathBuf::from(h).join(".config"))
            .unwrap_or_else(|_| PathBuf::from("/home/default/.config"))
    }
}

pub fn get_minecraft_data_path() -> PathBuf {
    get_app_data_path().join("MinecraftPE_Netease")
}

pub fn get_games_com_netease_path() -> PathBuf {
    get_minecraft_data_path().join("games/com.netease")
}

pub fn get_minecraft_worlds_path() -> PathBuf {
    get_minecraft_data_path().join("minecraftWorlds")
}

pub fn get_behavior_packs_path() -> PathBuf {
    get_games_com_netease_path().join("behavior_packs")
}

pub fn get_resource_packs_path() -> PathBuf {
    get_games_com_netease_path().join("resource_packs")
}

pub fn get_dependencies_packs_path() -> PathBuf {
    get_games_com_netease_path().join("_dependencies_packs")
}

#[cfg(windows)]
pub fn auto_search_mc_studio_download_game_path() -> Option<PathBuf> {
    use std::os::windows::fs::MetadataExt;

    for letter in b'A'..=b'Z' {
        let drive = format!("{}:", letter as char);
        let path = PathBuf::from(&drive).join("MCStudioDownload/game/MinecraftPE_Netease");
        
        if path.exists() {
            if let Ok(entries) = std::fs::read_dir(&path) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        let exe_path = entry_path.join("Minecraft.Windows.exe");
                        if exe_path.exists() {
                            return Some(path);
                        }
                    }
                }
            }
        }
    }
    None
}

#[cfg(not(windows))]
pub fn auto_search_mc_studio_download_game_path() -> Option<PathBuf> {
    None
}

pub fn create_junction(target: &Path, link: &Path) -> std::io::Result<bool> {
    if let Some(parent) = link.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    if link.exists() {
        if link.is_dir() {
            std::fs::remove_dir_all(link)?;
        } else {
            std::fs::remove_file(link)?;
        }
    }

    #[cfg(windows)]
    {
        #[link(name = "kernel32")]
        extern "system" {
            fn CreateSymbolicLinkW(lpSymlinkFileName: *const u16, lpTargetFileName: *const u16, dwFlags: u32) -> u32;
        }
        
        const SYMBOLIC_LINK_FLAG_DIRECTORY: u32 = 1;
        
        let target_str: Vec<u16> = target.to_string_lossy().encode_utf16().chain(std::iter::once(0)).collect();
        let link_str: Vec<u16> = link.to_string_lossy().encode_utf16().chain(std::iter::once(0)).collect();
        
        unsafe {
            let result = CreateSymbolicLinkW(link_str.as_ptr(), target_str.as_ptr(), SYMBOLIC_LINK_FLAG_DIRECTORY);
            Ok(result != 0)
        }
    }

    #[cfg(not(windows))]
    {
        std::os::unix::fs::symlink(target, link)?;
        Ok(true)
    }
}

pub fn clean_runtime_behavior_packs() -> std::io::Result<()> {
    let path = get_behavior_packs_path();
    if path.exists() {
        std::fs::remove_dir_all(&path)?;
        std::fs::create_dir_all(&path)?;
    }
    Ok(())
}

pub fn clean_runtime_resource_packs() -> std::io::Result<()> {
    let path = get_resource_packs_path();
    if path.exists() {
        std::fs::remove_dir_all(&path)?;
        std::fs::create_dir_all(&path)?;
    }
    Ok(())
}

pub fn clean_runtime_packs() -> std::io::Result<()> {
    clean_runtime_behavior_packs()?;
    clean_runtime_resource_packs()?;
    Ok(())
}