# vizier (`vz`)

`vizier` is a Rust CLI for structured desktop/system perception snapshots.
It is part of the OrbWeaver and Ponderer ecosystem, designed to give agents a sense of orientation into the system in which they live.
It emits JSON for two modes:
- `wake`: cold-start machine/user/environment orientation (`WakeObservation`)
- `snapshot` / `watch`: live state tracking (`Observation`)

## Status

Current implementation includes:
- Full CLI surface from the spec (`wake`, `snapshot`, `watch`, `--interval`, `--diff`, `--pretty`)
- Bare `vz` defaults to a one-shot compact snapshot
- Default compact wake output with `--verbose` full wake override
- Wake recent files are ranked by freshest available file activity (create/access/modify)
- Versioned schema structs for `WakeObservation` and `Observation`
- Diff streaming via RFC 6902 JSON Patch envelopes
- Filesystem delta events via `notify`
- macOS backend with baseline fallback and macOS enrichments
- Linux backend with baseline fallback and Hyprland IPC enrichment
- CLI/schema/stream integration tests

## Usage

```bash
vz
vz wake
vz snapshot
vz watch
vz watch --interval 250
vz watch --diff
vz --pretty snapshot
vz --no-public-ip wake
vz --verbose wake
vz --all-connections snapshot
vz --watch-path /tmp watch --diff
```

`vz` without a subcommand behaves like a compact `vz snapshot` (not a stream). Duplicate active connections from the same app/process are grouped with `connection_count` when needed.

All JSON goes to stdout. Errors go to stderr.

## Build From Source

```bash
cargo build
cargo build --release
```

Binary path:
- debug: `target/debug/vz`
- release: `target/release/vz`

## Install

Preferred install path (no Rust required):

```bash
curl -fsSL https://raw.githubusercontent.com/MLTQ/vizier/master/scripts/install.sh | sh
```

Install a specific tagged release:

```bash
curl -fsSL https://raw.githubusercontent.com/MLTQ/vizier/master/scripts/install.sh | VZ_VERSION=v0.1.0 sh
```

Install the latest rolling branch build:

```bash
curl -fsSL https://raw.githubusercontent.com/MLTQ/vizier/master/scripts/install.sh | VZ_VERSION=rolling sh
```

The release workflow publishes prebuilt archives for:
- `x86_64-unknown-linux-gnu`
- `aarch64-apple-darwin`

macOS release builds are Apple Silicon only.

If you built from source, install the compiled binary by copying it onto your `PATH`:

```bash
cargo build --release
mkdir -p ~/.local/bin
cp target/release/vz ~/.local/bin/vz
chmod +x ~/.local/bin/vz
```

If you already have a prebuilt `vz` binary, Cargo is not needed:

```bash
mkdir -p ~/.local/bin
cp ./vz ~/.local/bin/vz
chmod +x ~/.local/bin/vz
```

Make sure your shell can find it:

```bash
grep -q 'HOME/.local/bin' ~/.zshrc || echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
exec zsh
```

GitHub Actions builds release artifacts for macOS and Linux on every run. Pushes to `main`/`master` refresh a rolling prerelease tagged `rolling`, and version tags (`v*`) publish versioned GitHub Releases. Pull requests also produce downloadable CI artifacts.

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
- bare `vz` defaulting to snapshot
- CLI help command surface (`Usage: vz ...`)

## Design Notes

- Collectors are best-effort and fail open to preserve command reliability.
- Platform collectors layer on top of a shared baseline collector.
- `watch --diff` emits one full snapshot first, then patch envelopes.
- Live `fs_events` report create/modify/delete/rename and include best-effort file activity timestamps when the path still exists.
