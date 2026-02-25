use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use serde_json::Value;

fn bin() -> PathBuf {
    if let Some(path) = option_env!("CARGO_BIN_EXE_vizier") {
        return PathBuf::from(path);
    }

    let mut path = std::env::current_exe().expect("current exe path should be available");
    path.pop();
    if path.ends_with("deps") {
        path.pop();
    }
    path.push("vizier");

    path
}

#[test]
fn help_uses_vz_command_name() {
    let output = Command::new(bin())
        .arg("--help")
        .output()
        .expect("help invocation should succeed");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");
    assert!(stdout.contains("Usage: vz [OPTIONS] <COMMAND>"));
}

#[test]
fn wake_no_public_ip_omits_public_ip_field() {
    let output = Command::new(bin())
        .args(["--no-public-ip", "wake"])
        .output()
        .expect("wake invocation should succeed");

    assert!(output.status.success());

    let value: Value = serde_json::from_slice(&output.stdout).expect("wake should emit valid json");
    let network_identity = value
        .get("network_identity")
        .and_then(|value| value.as_object())
        .expect("network_identity object should exist");

    assert!(!network_identity.contains_key("public_ip"));
}

#[test]
fn wake_defaults_to_compact_profile() {
    let output = Command::new(bin())
        .args(["--no-public-ip", "wake"])
        .output()
        .expect("wake invocation should succeed");

    assert!(output.status.success());

    let value: Value = serde_json::from_slice(&output.stdout).expect("wake should emit valid json");

    let groups_len = value
        .get("user")
        .and_then(|value| value.get("groups"))
        .and_then(|value| value.as_array())
        .map(|value| value.len())
        .unwrap_or(0);
    assert!(groups_len <= 2);

    let history_len = value
        .get("recent_activity")
        .and_then(|value| value.get("shell_history"))
        .and_then(|value| value.as_array())
        .map(|value| value.len())
        .unwrap_or(0);
    assert!(history_len <= 5);

    let home_tree_is_omitted = value
        .get("filesystem")
        .and_then(|value| value.as_object())
        .map(|filesystem| !filesystem.contains_key("home_tree"))
        .unwrap_or(false);
    assert!(home_tree_is_omitted);

    let recent_files_len = value
        .get("filesystem")
        .and_then(|value| value.get("recent_files"))
        .and_then(|value| value.as_array())
        .map(|value| value.len())
        .unwrap_or(0);
    assert!(recent_files_len <= 5);
}

#[test]
fn wake_verbose_returns_larger_payload() {
    let compact = Command::new(bin())
        .args(["--no-public-ip", "wake"])
        .output()
        .expect("compact wake invocation should succeed");
    assert!(compact.status.success());

    let verbose = Command::new(bin())
        .args(["--no-public-ip", "--verbose", "wake"])
        .output()
        .expect("verbose wake invocation should succeed");
    assert!(verbose.status.success());

    assert!(verbose.stdout.len() > compact.stdout.len());
}

#[test]
fn snapshot_all_connections_is_superset() {
    let default_output = Command::new(bin())
        .arg("snapshot")
        .output()
        .expect("default snapshot should succeed");
    assert!(default_output.status.success());

    let all_output = Command::new(bin())
        .args(["--all-connections", "snapshot"])
        .output()
        .expect("all-connections snapshot should succeed");
    assert!(all_output.status.success());

    let default_json: Value =
        serde_json::from_slice(&default_output.stdout).expect("default snapshot should emit json");
    let all_json: Value = serde_json::from_slice(&all_output.stdout)
        .expect("all-connections snapshot should emit json");

    let default_count = default_json
        .get("net_connections")
        .and_then(|value| value.as_array())
        .map(|x| x.len())
        .unwrap_or(0);

    let all_count = all_json
        .get("net_connections")
        .and_then(|value| value.as_array())
        .map(|x| x.len())
        .unwrap_or(0);

    assert!(all_count >= default_count);
}

#[test]
fn watch_diff_emits_full_snapshot_then_patch() {
    let mut child = Command::new(bin())
        .args([
            "--watch-path",
            "/tmp",
            "watch",
            "--diff",
            "--interval",
            "100",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("watch process should start");

    let stdout = child.stdout.take().expect("stdout should be piped");
    let mut reader = BufReader::new(stdout);

    let mut line1 = String::new();
    reader
        .read_line(&mut line1)
        .expect("first watch line should be readable");
    let mut line2 = String::new();
    reader
        .read_line(&mut line2)
        .expect("second watch line should be readable");

    let _ = child.kill();
    let _ = child.wait();

    let snapshot: Value = serde_json::from_str(line1.trim()).expect("first line should be json");
    let patch: Value = serde_json::from_str(line2.trim()).expect("second line should be json");

    assert_eq!(
        snapshot.get("schema_version").and_then(|x| x.as_u64()),
        Some(1)
    );
    assert!(
        patch
            .get("patch")
            .and_then(|value| value.as_array())
            .is_some()
    );
}
