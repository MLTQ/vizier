# build-release.yml

## Purpose
Builds distributable `vz` binaries for supported macOS and Linux targets in GitHub Actions. It also publishes tagged builds as GitHub Release assets so end users can install without Rust tooling.

## Components

### `build` job
- **Does**: Compiles release binaries for Linux x86_64, macOS x86_64, and macOS arm64.
- **Interacts with**: Cargo, GitHub-hosted runners, and the repo's `Cargo.toml` binary target.

### Artifact packaging
- **Does**: Produces `tar.gz` archives and SHA-256 checksum files for each target.
- **Interacts with**: `tar`, `shasum`, `actions/upload-artifact`.

### Release publishing
- **Does**: Uploads packaged archives to GitHub Releases when the workflow runs on a `v*` tag.
- **Interacts with**: `softprops/action-gh-release`.

## Contracts

| Dependent | Expects | Breaking changes |
|-----------|---------|------------------|
| End users | Release assets named `vz-<target>.tar.gz` exist for supported targets | Renaming assets, removing targets |
| `scripts/install.sh` | Release asset names match the target triples it selects | Changing archive names or checksum naming |
| Maintainers | Push/PR builds surface packaging failures before release | Removing build matrix coverage |

## Notes
The workflow uploads CI artifacts on every push and pull request, but only publishes GitHub Release assets for version tags matching `v*`.
