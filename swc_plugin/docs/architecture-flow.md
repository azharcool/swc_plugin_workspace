# Architecture Flow

This document describes the deep execution flow for the plugin from entry point to transformed module output.

## 1. Entry Points

### Plugin runtime entry

File: `src/lib.rs`

- `process_program` receives `Program` plus SWC metadata.
- Reads transform plugin config JSON and deserializes into `PluginConfig`.
- Retrieves filename metadata from `TransformPluginMetadataContextKind::Filename`.
- Calls `run_pipeline(module, plugin_config, filename)` only for module programs with filename present.

### Fixture test entry

File: `tests/fixture.rs`

- Loads config from `tests/config.json`.
- Loads logical filename from per-case `fixture.json` when present.
- Executes `run_pipeline` through `PluginProcess` visit pass.
- Compares transformed output to golden `output.js`.

## 2. Pipeline Gate and Ordering

File: `src/core/pipeline.rs`

- Iterates all `theme_config` and nested `theme_mappings`.
- Selects mapping by suffix rule:
  - `filename.ends_with(theme_mapping.file)`
- If no mapping matches, pipeline returns without changes.

Execution order is fixed:

1. Analyze phase: `AnalyzeVisitor`
2. Transform phase: `TransformVisitor`

Analyzer must run first because transformer depends on `AnalyzeState` computed from module inspection and config materialization.

## 3. Analyze Phase

Files:

- `src/analyze/visitor.rs`
- `src/analyze/state.rs`

### 3.1 Resolver setup by directive

`visit_module` selects resolver branch using `theme_mapping.directive`:

- server path for `server` and `server-only`
- client path for `client` and `client-only`

For chosen branch:

- Creates resolver import if import source not already present.
- Creates variable declaration (for example, `const themeName = await getThemeCookieServer()`).
- Stores both into `AnalyzeState`.

### 3.2 Target and theme materialization

For each target in mapping:

- For `TargetNature::Component`:
  - Captures target specifier for later import detection.
  - Builds themed import for base target symbol aliasing with `__{theme}` suffix.
  - Builds wrapper function identity: `ThemeWrapper__{ComponentName}`.
  - Builds target fallback JSX and theme-specific JSX elements.
  - Builds `if (themeName === "{theme}") return <Component__theme {...props} />` statements.
  - Composes final async wrapper body:
    1. resolver variable declaration
    2. theme condition statements
    3. fallback return to original target component

- For `TargetNature::Agents` and `TargetNature::Page`:
  - currently logs discovery paths without equivalent transformation output logic.

### 3.3 Import scan

`visit_import_decl` checks whether target specifier appears in module imports and records component presence in state.

## 4. Transform Phase

File: `src/transform/visitor.rs`

`visit_mut_module` performs module-level edits using analyzer state:

1. Insert resolver import after last import.
2. Remove original target import containing target symbol.
3. Insert themed imports.
4. Append generated wrapper function declaration at module end.

`visit_mut_jsx_element` performs JSX rewrite:

- Replaces opening and closing tag identifiers matching target symbol with wrapper ident.
- Skips already replaced nodes.

## 5. End-to-End Example

Fixture inputs in:

- `tests/fixture/home/input.tsx`
- `tests/fixture/signup/input.tsx`

Typical resulting changes in outputs:

- Add resolver import.
- Add themed imports with aliased local symbols.
- Replace `<Target ... />` with `<ThemeWrapper__Target ... />`.
- Append async wrapper function that returns themed component based on resolved theme value.

## 6. Early Return and Risk Paths

- No filename metadata: pipeline not called from runtime entry.
- No mapping suffix match: pipeline exits without AST mutation.
- Config default/fallback parse path: plugin uses `PluginConfig::default()`.

## 7. Known Gaps

Observed from current source:

- `TargetNature::Agents` and `TargetNature::Page` are modeled but not transformed like `Component`.
- `src/config/resolver.rs` is currently empty.
- `src/transform/builders.rs` appears not wired into the active visitor flow.
