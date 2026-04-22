# Contributor Workflow

This guide covers day-to-day development, fixture testing, and debug workflow for `swc_plugin`.

## 1. Build and Test Loop

From workspace root:

```bash
cargo build --manifest-path swc_plugin/Cargo.toml
cargo test --manifest-path swc_plugin/Cargo.toml --test fixture
```

Use full workspace checks when needed:

```bash
cargo build
cargo test
```

## 2. Fixture Test Structure

Fixture harness: `tests/fixture.rs`.

Each scenario folder under `tests/fixture/` should contain:

- `input.tsx`: source file before plugin rewrite.
- `output.js`: expected transformed output.
- `fixture.json`: logical filename metadata used for config matching.

Examples:

- `tests/fixture/home/`
- `tests/fixture/signup/`

## 3. Why fixture.json matters

`fixture.json` contains:

```json
{ "filename": "apps/webstore/src/app/[locale]/home/page.tsx" }
```

The plugin uses this logical path, not fixture file disk location, for mapping selection.

Because matching is suffix-based (`ends_with`), keep the suffix aligned with `themeMappings.file` values in config.

## 4. Adding a New Fixture Scenario

1. Create a new folder under `tests/fixture/<scenario>/`.
2. Add `input.tsx` containing target component usage expected to be rewritten.
3. Add `fixture.json` with a filename suffix that maps to your new or existing config entry.
4. Add `output.js` with expected transformed code:
   - resolver import
   - themed imports
   - wrapper function
   - rewritten JSX usage
5. Run fixture test command and iterate until output matches expected file.

## 5. Updating tests/config.json Safely

When editing `tests/config.json`:

- Keep JSON keys in camelCase (`themeConfig`, `themeNameResolver`, `themeMappings`, `themeName`).
- Verify each mapping has:
  - `file` suffix
  - `directive`
  - `targets` with import specifiers
  - `themes` for alternate variants
- Ensure specifier nature matches desired import form.

## 6. Debug Logging

Logger helper: `src/debug/file_logger.rs`.

Behavior:

- Initializes once via `OnceLock`.
- Writes debug logs to `plugin_debugging.log` in current working directory.
- Fixture test currently initializes logger in `tests/fixture.rs`.

If you need additional diagnostics, add `debug!` logs in analyzer or transformer paths and rerun fixture tests.

## 7. Contribution Checklist

Before opening changes:

1. Build plugin crate successfully.
2. Run fixture test suite.
3. Confirm expected output fixtures reflect intentional changes only.
4. Verify no unintended mapping regressions due to suffix collisions.
5. Update docs when behavior or config contracts change.

## 8. Known Implementation Status

Current observations from source:

- Component target transformation path is implemented.
- Agents/Page target natures are modeled but do not yet have equivalent transformation behavior.
- Some files appear reserved or partially wired (`src/config/resolver.rs`, `src/transform/builders.rs`).
