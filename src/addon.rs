use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum PackType {
    #[default]
    Behavior,
    Resource,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackInfo {
    pub name: String,
    pub uuid: String,
    pub version: String,
    pub path: PathBuf,
    #[serde(default)]
    pub pack_type: PackType,
}

impl PackInfo {
    pub fn is_valid(&self) -> bool {
        !self.uuid.is_empty() && !self.version.is_empty() && self.pack_type != PackType::Unknown
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeteasePackInfo {
    pub base_info: PackInfo,
    pub dependencies: Vec<String>,
}

impl NeteasePackInfo {
    pub fn is_valid(&self) -> bool {
        self.base_info.is_valid()
    }
}

#[derive(Debug, Deserialize)]
struct ManifestHeader {
    pub name: String,
    pub uuid: String,
    pub version: Vec<u32>,
}

#[derive(Debug, Deserialize)]
struct ManifestModule {
    #[serde(rename = "type")]
    pub module_type: String,
    pub uuid: Option<String>,
    pub version: Option<Vec<u32>>,
}

#[derive(Debug, Deserialize)]
struct Manifest {
    pub header: ManifestHeader,
    pub modules: Vec<ManifestModule>,
}

pub fn parse_json_pack_info(json_content: &str) -> Option<PackInfo> {
    let manifest: Manifest = serde_json::from_str(json_content).ok()?;
    
    let pack_type = if manifest.modules.iter().any(|m| m.module_type == "data") {
        PackType::Behavior
    } else if manifest.modules.iter().any(|m| m.module_type == "resources") {
        PackType::Resource
    } else {
        PackType::Unknown
    };
    
    let version = manifest.header.version
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(".");
    
    Some(PackInfo {
        name: manifest.header.name,
        uuid: manifest.header.uuid,
        version,
        path: PathBuf::new(),
        pack_type,
    })
}

pub fn parse_pack_info(pack_path: &Path) -> Option<PackInfo> {
    let manifest_path = pack_path.join("manifest.json");
    if !manifest_path.exists() {
        return None;
    }
    
    let content = std::fs::read_to_string(&manifest_path).ok()?;
    let mut pack_info = parse_json_pack_info(&content)?;
    pack_info.path = pack_path.to_path_buf();
    Some(pack_info)
}

pub fn parse_netease_pack_info(pack_path: &Path) -> Option<NeteasePackInfo> {
    let base_info = parse_pack_info(pack_path)?;
    let mut dependencies = Vec::new();
    
    let pack_config_path = pack_path.join("pack_config.json");
    if pack_config_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&pack_config_path) {
            if let Ok(config) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(deps) = config.get("dependencies").and_then(|d| d.as_array()) {
                    for dep in deps {
                        if let Some(uuid) = dep.get("uuid").and_then(|u| u.as_str()) {
                            dependencies.push(uuid.to_string());
                        }
                    }
                }
            }
        }
    }
    
    Some(NeteasePackInfo {
        base_info,
        dependencies,
    })
}

pub fn create_empty_addon_manifest(name: &str, version: &[u32; 3]) -> (String, String) {
    let uuid = uuid::Uuid::new_v4().to_string();
    let version_str = format!("{}.{}.{}", version[0], version[1], version[2]);
    let pack_name = if name.is_empty() {
        format!("{} Pack", uuid_simple())
    } else {
        name.to_string()
    };
    
    let behavior_manifest = format!(r#"{{
  "format_version": 2,
  "header": {{
    "name": "{}",
    "uuid": "{}",
    "version": [{}, {}, {}],
    "description": "Behavior pack created by MCDK"
  }},
  "modules": [
    {{
      "type": "data",
      "uuid": "{}",
      "version": [{}, {}, {}]
    }}
  ]
}}"#, 
        pack_name, uuid,
        version[0], version[1], version[2],
        uuid::Uuid::new_v4().simple(),
        version[0], version[1], version[2]
    );
    
    let resource_uuid = uuid::Uuid::new_v4().to_string();
    let resource_name = format!("{} Resource", name);
    
    let resource_manifest = format!(r#"{{
  "format_version": 2,
  "header": {{
    "name": "{}",
    "uuid": "{}",
    "version": [{}, {}, {}],
    "description": "Resource pack created by MCDK"
  }},
  "modules": [
    {{
      "type": "resources",
      "uuid": "{}",
      "version": [{}, {}, {}]
    }}
  ]
}}"#,
        resource_name, resource_uuid,
        version[0], version[1], version[2],
        resource_uuid,
        version[0], version[1], version[2]
    );
    
    (behavior_manifest, resource_manifest)
}

fn uuid_simple() -> String {
    uuid::Uuid::new_v4().simple().to_string()
}