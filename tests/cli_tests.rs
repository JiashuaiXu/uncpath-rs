use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_help_output() {
    let mut cmd = Command::cargo_bin("uncpath").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Convert UNC paths to POSIX paths"));
}

#[test]
fn test_version_output() {
    let mut cmd = Command::cargo_bin("uncpath").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("uncpath"));
}

#[test]
fn test_convert_windows_unc_with_defaults() {
    let mut cmd = Command::cargo_bin("uncpath").unwrap();
    cmd.arg(r"\\server\shared\documents\file.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("/mnt/shared/documents/file.txt"));
}

#[test]
fn test_convert_unix_style_with_defaults() {
    let mut cmd = Command::cargo_bin("uncpath").unwrap();
    cmd.arg("//nas/data/report.pdf")
        .assert()
        .success()
        .stdout(predicate::str::contains("/mnt/nas/report.pdf"));
}

#[test]
fn test_convert_smb_url_with_defaults() {
    let mut cmd = Command::cargo_bin("uncpath").unwrap();
    cmd.arg("smb://nas/data/folder/file.doc")
        .assert()
        .success()
        .stdout(predicate::str::contains("/mnt/nas/folder/file.doc"));
}

#[test]
fn test_custom_mapping_from_cli() {
    let mut cmd = Command::cargo_bin("uncpath").unwrap();
    cmd.arg("--mapping")
        .arg("myhost:myshare:/custom/mount")
        .arg(r"\\myhost\myshare\test.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("/custom/mount/test.txt"));
}

#[test]
fn test_multiple_custom_mappings() {
    let mut cmd = Command::cargo_bin("uncpath").unwrap();
    cmd.arg("--mapping")
        .arg("host1:share1:/mount1")
        .arg("--mapping")
        .arg("host2:share2:/mount2")
        .arg(r"\\host2\share2\file.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("/mount2/file.txt"));
}

#[test]
fn test_list_mappings() {
    let mut cmd = Command::cargo_bin("uncpath").unwrap();
    cmd.arg("--list")
        .arg("//dummy/path")
        .assert()
        .success()
        .stdout(predicate::str::contains("Configured mappings:"))
        .stdout(predicate::str::contains(r"\\server\shared -> /mnt/shared"))
        .stdout(predicate::str::contains(r"\\nas\data -> /mnt/nas"));
}

#[test]
fn test_no_defaults_flag() {
    let mut cmd = Command::cargo_bin("uncpath").unwrap();
    cmd.arg("--no-defaults")
        .arg("--mapping")
        .arg("custom:share:/custom")
        .arg(r"\\custom\share\file.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("/custom/file.txt"));
}

#[test]
fn test_unmapped_host_share_error() {
    let mut cmd = Command::cargo_bin("uncpath").unwrap();
    cmd.arg("--no-defaults")
        .arg(r"\\unknown\share\path")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No mapping found"));
}

#[test]
fn test_invalid_path_format() {
    let mut cmd = Command::cargo_bin("uncpath").unwrap();
    cmd.arg("/single/slash/path")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "does not match any supported UNC format",
        ));
}

#[test]
fn test_invalid_cli_mapping_format() {
    let mut cmd = Command::cargo_bin("uncpath").unwrap();
    cmd.arg("--mapping")
        .arg("invalid:format")
        .arg(r"\\server\share\path")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Expected format: host:share:mount_point",
        ));
}

#[test]
fn test_load_mappings_from_file() {
    // Create a temporary JSON file with mappings
    let temp_dir = std::env::temp_dir();
    let mapping_file = temp_dir.join("test_mappings.json");

    let mappings_json = r#"[
        {"host": "testhost", "share": "testshare", "mount_point": "/test/mount"}
    ]"#;

    fs::write(&mapping_file, mappings_json).unwrap();

    let mut cmd = Command::cargo_bin("uncpath").unwrap();
    cmd.arg("--no-defaults")
        .arg("--file")
        .arg(&mapping_file)
        .arg(r"\\testhost\testshare\document.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("/test/mount/document.txt"));

    // Cleanup
    fs::remove_file(&mapping_file).ok();
}

#[test]
fn test_case_insensitive_host_share() {
    let mut cmd = Command::cargo_bin("uncpath").unwrap();
    cmd.arg(r"\\SERVER\SHARED\file.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("/mnt/shared/file.txt"));
}

#[test]
fn test_root_path_conversion() {
    let mut cmd = Command::cargo_bin("uncpath").unwrap();
    cmd.arg(r"\\server\shared")
        .assert()
        .success()
        .stdout(predicate::str::contains("/mnt/shared"));
}
