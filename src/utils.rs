use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    components: Vec<u32>,
}

impl Version {
    pub fn new(version_str: &str) -> Option<Self> {
        let components: Vec<u32> = version_str
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        
        if components.is_empty() {
            None
        } else {
            Some(Self { components })
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let max_len = self.components.len().max(other.components.len());
        
        for i in 0..max_len {
            let self_comp = self.components.get(i).unwrap_or(&0);
            let other_comp = other.components.get(i).unwrap_or(&0);
            
            if self_comp != other_comp {
                return Some(self_comp.cmp(other_comp));
            }
        }
        Some(std::cmp::Ordering::Equal)
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Equal)
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.components.iter().map(|v| v.to_string()).collect::<Vec<_>>().join("."))
    }
}

pub fn create_random_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn create_compact_uuid() -> String {
    uuid::Uuid::new_v4().simple().to_string()
}

#[cfg(windows)]
pub fn path_to_utf8<P: AsRef<Path>>(p: P) -> String {
    use std::os::windows::ffi::OsStrExt;
    let w: Vec<u16> = p.as_ref().as_os_str().encode_wide().collect();
    String::from_utf16_lossy(&w)
}

#[cfg(not(windows))]
pub fn path_to_utf8<P: AsRef<Path>>(p: P) -> String {
    p.as_ref().to_string_lossy().into_owned()
}

#[cfg(windows)]
pub fn path_to_generic_utf8<P: AsRef<Path>>(p: P) -> String {
    path_to_utf8(p)
}

#[cfg(not(windows))]
pub fn path_to_generic_utf8<P: AsRef<Path>>(p: P) -> String {
    p.as_ref().to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let v1 = Version::new("1.2.3").unwrap();
        let v2 = Version::new("1.2.4").unwrap();
        assert!(v1 < v2);
        
        let v3 = Version::new("2.0.0").unwrap();
        assert!(v1 < v3);
    }

    #[test]
    fn test_uuid() {
        let uuid = create_random_uuid();
        assert_eq!(uuid.len(), 36);
        
        let compact = create_compact_uuid();
        assert_eq!(compact.len(), 32);
    }
}