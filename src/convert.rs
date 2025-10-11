use crate::errors::{Result, UncPathError};
use crate::mapping::MappingTable;
use once_cell::sync::Lazy;
use regex::Regex;

// Compile regex patterns once at startup
static WINDOWS_UNC_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\\\\([^\\]+)\\([^\\]+)(.*)$").unwrap());
static SMB_URL_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^smb://([^/]+)/([^/]+)(.*)$").unwrap());
static UNIX_STYLE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^//([^/]+)/([^/]+)(.*)$").unwrap());

/// Parsed UNC path components
#[derive(Debug, PartialEq)]
pub struct UncPath {
    pub host: String,
    pub share: String,
    pub path: String,
}

impl UncPath {
    pub fn new(host: String, share: String, path: String) -> Self {
        Self { host, share, path }
    }
}

/// Parse a UNC path from various formats:
/// - \\host\share\path (Windows UNC)
/// - //host/share/path (Unix-style)
/// - smb://host/share/path (SMB URL)
pub fn parse_unc_path(input: &str) -> Result<UncPath> {
    let input = input.trim();

    // Try Windows UNC format: \\host\share\path
    if input.starts_with("\\\\") {
        return parse_windows_unc(input);
    }

    // Try SMB URL format: smb://host/share/path
    if input.starts_with("smb://") {
        return parse_smb_url(input);
    }

    // Try Unix-style format: //host/share/path
    if input.starts_with("//") {
        return parse_unix_style(input);
    }

    Err(UncPathError::InvalidFormat(format!(
        "Path does not match any supported UNC format: {}",
        input
    )))
}

/// Parse Windows UNC path: \\host\share\path
fn parse_windows_unc(input: &str) -> Result<UncPath> {
    if let Some(caps) = WINDOWS_UNC_RE.captures(input) {
        let host = caps.get(1).unwrap().as_str().to_string();
        let share = caps.get(2).unwrap().as_str().to_string();
        let path = caps
            .get(3)
            .map(|m| m.as_str())
            .unwrap_or("")
            .replace('\\', "/");

        Ok(UncPath::new(host, share, path))
    } else {
        Err(UncPathError::InvalidFormat(format!(
            "Invalid Windows UNC format: {}",
            input
        )))
    }
}

/// Parse SMB URL: smb://host/share/path
fn parse_smb_url(input: &str) -> Result<UncPath> {
    if let Some(caps) = SMB_URL_RE.captures(input) {
        let host = caps.get(1).unwrap().as_str().to_string();
        let share = caps.get(2).unwrap().as_str().to_string();
        let path = caps.get(3).map(|m| m.as_str()).unwrap_or("").to_string();

        Ok(UncPath::new(host, share, path))
    } else {
        Err(UncPathError::InvalidFormat(format!(
            "Invalid SMB URL format: {}",
            input
        )))
    }
}

/// Parse Unix-style UNC: //host/share/path
fn parse_unix_style(input: &str) -> Result<UncPath> {
    if let Some(caps) = UNIX_STYLE_RE.captures(input) {
        let host = caps.get(1).unwrap().as_str().to_string();
        let share = caps.get(2).unwrap().as_str().to_string();
        let path = caps.get(3).map(|m| m.as_str()).unwrap_or("").to_string();

        Ok(UncPath::new(host, share, path))
    } else {
        Err(UncPathError::InvalidFormat(format!(
            "Invalid Unix-style UNC format: {}",
            input
        )))
    }
}

/// Convert UNC path to POSIX path using mapping table
pub fn convert_to_posix(input: &str, mapping_table: &MappingTable) -> Result<String> {
    let unc_path = parse_unc_path(input)?;

    if let Some(mount_point) = mapping_table.find_mount_point(&unc_path.host, &unc_path.share) {
        let posix_path = if unc_path.path.is_empty() || unc_path.path == "/" {
            mount_point.to_string()
        } else {
            format!("{}{}", mount_point.trim_end_matches('/'), unc_path.path)
        };
        Ok(posix_path)
    } else {
        Err(UncPathError::MappingNotFound(unc_path.host, unc_path.share))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_windows_unc() {
        let result = parse_unc_path(r"\\server\share\folder\file.txt").unwrap();
        assert_eq!(result.host, "server");
        assert_eq!(result.share, "share");
        assert_eq!(result.path, "/folder/file.txt");
    }

    #[test]
    fn test_parse_windows_unc_no_path() {
        let result = parse_unc_path(r"\\server\share").unwrap();
        assert_eq!(result.host, "server");
        assert_eq!(result.share, "share");
        assert_eq!(result.path, "");
    }

    #[test]
    fn test_parse_smb_url() {
        let result = parse_unc_path("smb://server/share/folder/file.txt").unwrap();
        assert_eq!(result.host, "server");
        assert_eq!(result.share, "share");
        assert_eq!(result.path, "/folder/file.txt");
    }

    #[test]
    fn test_parse_unix_style() {
        let result = parse_unc_path("//server/share/folder/file.txt").unwrap();
        assert_eq!(result.host, "server");
        assert_eq!(result.share, "share");
        assert_eq!(result.path, "/folder/file.txt");
    }

    #[test]
    fn test_parse_invalid_format() {
        let result = parse_unc_path("/server/share/path");
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_to_posix() {
        let mut table = MappingTable::new();
        table.add_mapping("server", "shared", "/mnt/shared");

        let result = convert_to_posix(r"\\server\shared\folder\file.txt", &table).unwrap();
        assert_eq!(result, "/mnt/shared/folder/file.txt");
    }

    #[test]
    fn test_convert_smb_to_posix() {
        let mut table = MappingTable::new();
        table.add_mapping("nas", "data", "/mnt/nas");

        let result = convert_to_posix("smb://nas/data/documents/report.pdf", &table).unwrap();
        assert_eq!(result, "/mnt/nas/documents/report.pdf");
    }

    #[test]
    fn test_convert_no_mapping() {
        let table = MappingTable::new();
        let result = convert_to_posix(r"\\unknown\share\path", &table);
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_root_path() {
        let mut table = MappingTable::new();
        table.add_mapping("server", "shared", "/mnt/shared");

        let result = convert_to_posix(r"\\server\shared", &table).unwrap();
        assert_eq!(result, "/mnt/shared");
    }
}
