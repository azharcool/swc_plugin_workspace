# SWC Workspace

This repository contains a Rust workspace for an SWC transformation plugin and a local runner crate.

## Workspace Layout

- `swc_plugin/`: SWC plugin crate that analyzes and rewrites module AST based on theme mapping config.
- `runner/`: helper binary crate that depends on `swc_plugin` for local integration and experimentation.

## Quick Start

### Prerequisites

- Rust toolchain with Cargo.

### Build

```bash
cargo build
```

### Test

```bash
cargo test
```

## Targeted Commands

### Build plugin crate only

```bash
cargo build --manifest-path swc_plugin/Cargo.toml
```

### Run plugin fixture tests

```bash
cargo test --manifest-path swc_plugin/Cargo.toml --test fixture
```

### Build runner crate only

```bash
cargo build --manifest-path runner/Cargo.toml
```

## Fast Validation Path

Use this sequence when validating plugin behavior changes quickly:

1. `cargo build --manifest-path swc_plugin/Cargo.toml`
2. `cargo test --manifest-path swc_plugin/Cargo.toml --test fixture`

If fixture output changes, compare expected outputs under `swc_plugin/tests/fixture/*/output.js`.

## Documentation Index

- Plugin overview and usage: `swc_plugin/README.md`
- Deep architecture flow: `swc_plugin/docs/architecture-flow.md`
- Config schema and examples: `swc_plugin/docs/config-reference.md`
- Contributor workflow and fixtures: `swc_plugin/docs/contributor-workflow.md`
