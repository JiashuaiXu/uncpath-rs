use crate::errors::{Result, UncPathError};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountMapping {
    pub host: String,
    pub share: String,
    pub mount_point: String,
}

#[derive(Debug, Clone)]
pub struct MappingTable {
    mappings: Vec<MountMapping>,
}

impl MappingTable {
    /// Create a new empty mapping table
    pub fn new() -> Self {
        Self {
            mappings: Vec::new(),
        }
    }

    /// Create mapping table with default mappings
    pub fn with_defaults() -> Self {
        let mut table = Self::new();
        // Add some common default mappings as examples
        table.add_mapping("server", "shared", "/mnt/shared");
        table.add_mapping("nas", "data", "/mnt/nas");
        table
    }

    /// Load mappings from environment variable UNCPATH_MAPPINGS
    /// Expected format: JSON array of mapping objects
    /// Example: [{"host":"server","share":"shared","mount_point":"/mnt/shared"}]
    pub fn load_from_env(&mut self) -> Result<()> {
        if let Ok(mappings_json) = env::var("UNCPATH_MAPPINGS") {
            let mappings: Vec<MountMapping> = serde_json::from_str(&mappings_json)?;
            for mapping in mappings {
                self.mappings.push(mapping);
            }
        }
        Ok(())
    }

    /// Load mappings from a JSON file
    pub fn load_from_file(&mut self, path: &PathBuf) -> Result<()> {
        let content = fs::read_to_string(path)?;
        let mappings: Vec<MountMapping> = serde_json::from_str(&content)?;
        for mapping in mappings {
            self.mappings.push(mapping);
        }
        Ok(())
    }

    /// Add a single mapping
    pub fn add_mapping(&mut self, host: &str, share: &str, mount_point: &str) {
        self.mappings.push(MountMapping {
            host: host.to_string(),
            share: share.to_string(),
            mount_point: mount_point.to_string(),
        });
    }

    /// Add mappings from command line arguments
    /// Format: host:share:mount_point
    pub fn add_from_cli(&mut self, mapping_str: &str) -> Result<()> {
        let parts: Vec<&str> = mapping_str.split(':').collect();
        if parts.len() != 3 {
            return Err(UncPathError::InvalidMapping(format!(
                "Expected format: host:share:mount_point, got: {}",
                mapping_str
            )));
        }
        self.add_mapping(parts[0], parts[1], parts[2]);
        Ok(())
    }

    /// Find mount point for given host and share
    pub fn find_mount_point(&self, host: &str, share: &str) -> Option<&str> {
        // Normalize host and share for case-insensitive comparison
        let host_lower = host.to_lowercase();
        let share_lower = share.to_lowercase();

        for mapping in &self.mappings {
            if mapping.host.to_lowercase() == host_lower
                && mapping.share.to_lowercase() == share_lower
            {
                return Some(&mapping.mount_point);
            }
        }
        None
    }

    /// Get all mappings
    pub fn get_mappings(&self) -> &[MountMapping] {
        &self.mappings
    }
}

impl Default for MappingTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_mapping() {
        let mut table = MappingTable::new();
        table.add_mapping("host1", "share1", "/mnt/test");
        assert_eq!(table.find_mount_point("host1", "share1"), Some("/mnt/test"));
    }

    #[test]
    fn test_case_insensitive_lookup() {
        let mut table = MappingTable::new();
        table.add_mapping("Host1", "Share1", "/mnt/test");
        assert_eq!(table.find_mount_point("host1", "share1"), Some("/mnt/test"));
        assert_eq!(table.find_mount_point("HOST1", "SHARE1"), Some("/mnt/test"));
    }

    #[test]
    fn test_add_from_cli() {
        let mut table = MappingTable::new();
        table.add_from_cli("host1:share1:/mnt/test").unwrap();
        assert_eq!(table.find_mount_point("host1", "share1"), Some("/mnt/test"));
    }

    #[test]
    fn test_add_from_cli_invalid() {
        let mut table = MappingTable::new();
        let result = table.add_from_cli("invalid:format");
        assert!(result.is_err());
    }
}
