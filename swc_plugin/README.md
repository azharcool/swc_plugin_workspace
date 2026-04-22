# swc_plugin

`swc_plugin` is an SWC transform plugin crate that rewrites target imports and JSX usage into theme-aware wrapper components.

## What It Does

For configured files, the plugin:

1. Detects the applicable theme mapping from file path suffix.
2. Chooses a resolver path (server or client) from mapping directive.
3. Injects resolver import and themed imports.
4. Generates an async wrapper component with conditional theme branches.
5. Rewrites target JSX element usage to the generated wrapper.

## Runtime Entry Contract

Entry point: `src/lib.rs`.

The plugin reads SWC transform metadata:

- Transform config: JSON string deserialized into `PluginConfig`.
- Filename context: required for mapping lookup.

Execution details:

- Runs only for `Program::Module`.
- If filename metadata is missing, pipeline does not run.
- If config parse fails, defaults are used.

## Pipeline Overview

Pipeline implementation: `src/core/pipeline.rs`.

1. Locate a `themeMappings` entry where `filename.ends_with(theme_mapping.file)`.
2. If no mapping matches, return without AST changes.
3. Run `AnalyzeVisitor` to build transformation state.
4. Run `TransformVisitor` to mutate module and JSX based on that state.

## Current Behavior Scope

Config model supports `TargetNature` values:

- `Component`
- `Agents`
- `Page`

Current implemented transformation path is primarily `Component` in analyzer/transformer logic. `Agents` and `Page` paths are present but not transformed in the current code.

## Test and Development Commands

From workspace root:

```bash
cargo build --manifest-path swc_plugin/Cargo.toml
cargo test --manifest-path swc_plugin/Cargo.toml --test fixture
```

## Learn More

- Architecture flow: `docs/architecture-flow.md`
- Config reference: `docs/config-reference.md`
- Contributor workflow: `docs/contributor-workflow.md`
