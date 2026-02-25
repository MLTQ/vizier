# cli_behavior.rs

## Purpose
Integration tests for CLI contract behavior and flag semantics. These tests guard invocation details (`vz` usage text, watch stream shape, and flag effects) independent of platform-specific collector internals.

## Components

### `help_uses_vz_command_name`
- **Does**: Verifies help text exposes the intended `vz` command name.
- **Interacts with**: Clap parser wiring in `main.rs`.

### `wake_no_public_ip_omits_public_ip_field`
- **Does**: Verifies wake output omits `network_identity.public_ip` when `--no-public-ip` is set.
- **Interacts with**: Wake config handling in `main.rs` and observer backends.

### `wake_defaults_to_compact_profile`
- **Does**: Verifies default wake output uses compacted content (bounded groups/history and omitted `filesystem.home_tree`).
- **Interacts with**: `WakeObservation::compact` in `observation.rs`.

### `wake_verbose_returns_larger_payload`
- **Does**: Verifies `--verbose` bypasses compacting and returns a larger wake payload.
- **Interacts with**: `--verbose` flag handling in `main.rs`.

### `snapshot_all_connections_is_superset`
- **Does**: Verifies `--all-connections` does not reduce observed network connection rows.
- **Interacts with**: Active connection collector in `util/net.rs`.

### `watch_diff_emits_full_snapshot_then_patch`
- **Does**: Verifies watch diff mode outputs an initial full snapshot followed by patch envelopes.
- **Interacts with**: Stream loop in `main.rs` and patch builder in `diff.rs`.

## Contracts

| Dependent | Expects | Breaking changes |
|-----------|---------|------------------|
| CLI users | Stable behavior for documented flags and stream format | Flag semantic changes without test updates |
| CI | Fast, deterministic CLI-level contract checks | Tests depending on non-deterministic timing or global machine state |
