# vizier (`vz`)

`vizier` is a Rust CLI for structured desktop/system perception snapshots.

It emits JSON for two modes:
- `wake`: cold-start machine/user/environment orientation (`WakeObservation`)
- `snapshot` / `watch`: live state tracking (`Observation`)

## Status

Current implementation includes:
- Full CLI surface from the spec (`wake`, `snapshot`, `watch`, `--interval`, `--diff`, `--pretty`)
- Versioned schema structs for `WakeObservation` and `Observation`
- Diff streaming via RFC 6902 JSON Patch envelopes
- Filesystem delta events via `notify`
- macOS backend with baseline fallback and macOS enrichments
- Linux backend with baseline fallback and Hyprland IPC enrichment
- CLI/schema/stream integration tests

## Usage

```bash
vz wake
vz snapshot
vz watch
vz watch --interval 250
vz watch --diff
vz --pretty snapshot
vz --no-public-ip wake
vz --all-connections snapshot
vz --watch-path /tmp watch --diff
```

All JSON goes to stdout. Errors go to stderr.

## Build

```bash
cargo build
cargo build --release
```

Binary path:
- debug: `target/debug/vizier`
- release: `target/release/vizier`

On this machine, release binary size is currently `3,927,584` bytes (`3.9 MB`), above the `<2 MB` long-term target in the spec.

## Tests

```bash
cargo test
```

Current integration test coverage includes:
- schema shape checks
- wake `--no-public-ip` behavior
- snapshot `--all-connections` behavior
- watch `--diff` stream contract
- CLI help command surface (`Usage: vz ...`)

## Design Notes

- Collectors are best-effort and fail open to preserve command reliability.
- Platform collectors layer on top of a shared baseline collector.
- `watch --diff` emits one full snapshot first, then patch envelopes.
