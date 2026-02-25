# main.rs

## Purpose
CLI entry point for `vz`. Parses arguments, dispatches commands, and serializes observation output to stdout while preserving stderr and exit-code behavior.

## Components

### `Cli`
- **Does**: Defines global flags and subcommands exposed by the binary.
- **Interacts with**: `create_observer` and `create_waker` in `observer/mod.rs`.

### `run`
- **Does**: Executes one-shot (`wake`, `snapshot`) and streaming (`watch`) flows.
- **Interacts with**: `create_diff_envelope` in `diff.rs`, schema types in `observation.rs`.

### `print_json`
- **Does**: Emits JSON in pretty or compact form and flushes immediately for stream consumers.
- **Interacts with**: `serde_json` serializer.

## Contracts

| Dependent | Expects | Breaking changes |
|-----------|---------|------------------|
| CLI users | `vz wake`, `vz snapshot`, `vz watch` exist and output JSON | Command names/flags, output format |
| Scripts | stdout emits JSON lines and stderr emits errors | Mixing logs into stdout |

## Notes
Initial implementation uses platform baseline collectors and is structured to be replaced by richer per-OS backends.
