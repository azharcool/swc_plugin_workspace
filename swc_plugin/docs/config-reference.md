# Config Reference

Configuration is deserialized into `PluginConfig` from transform metadata JSON.

Source schema: `src/config/models.rs`.
Example config: `tests/config.json`.

## Top-Level Shape

```json
{
  "themeConfig": [],
  "themeNameResolver": {},
  "debug": true
}
```

## PluginConfig

### debug

- Type: `boolean | null`
- Purpose: debug intent flag in config payload.
- Notes: plugin logging is initialized in tests through logger helper; this flag is part of payload schema.

### themeConfig

- Type: `ThemeConfig[]`
- JSON key: `themeConfig`
- Purpose: list of target groups and mapping rules.

### themeNameResolver

- Type: `ThemeNameResolver | null`
- JSON key: `themeNameResolver`
- Purpose: resolver settings for server/client branches.

## ThemeConfig

```json
{
  "target": "base-package",
  "description": "Override only happen in base-package",
  "themeMappings": []
}
```

Fields:

- `target`: logical config grouping label.
- `description`: free-form description.
- `themeMappings`: list of mapping rules.

## ThemeMapping

```json
{
  "file": "signup/page.tsx",
  "nature": "Page",
  "directive": "server",
  "targets": []
}
```

Fields:

- `file`: suffix pattern matched with `filename.ends_with(file)`.
- `nature`: `Component | Agents | Page`.
- `directive`: `server | client | server-only | client-only`.
- `targets`: list of target rewrite descriptors.

Important behavior:

- Matching is suffix-based, not full-path exact matching.

## Target and Theme

`Target` and `Theme` share most fields:

```json
{
  "nature": "Component",
  "directive": "client",
  "themeName": "base",
  "package": "base",
  "action": "",
  "import": {
    "source": "./client",
    "specifiers": [
      {
        "nature": "ImportDefaultSpecifier",
        "value": "Client"
      }
    ]
  }
}
```

Field notes:

- `themeName`: used to alias generated import locals as `{symbol}__{themeName}`.
- `import`: source and specifiers for generated import declarations.
- `themes`: in `Target`, list of additional themed alternatives.

## Import and Specifier

### Import

```json
{
  "source": "@znode/utils/theme-resolver/theme-resolver.server",
  "specifiers": [
    { "nature": "ImportDefaultSpecifier", "value": "getThemeCookieServer" }
  ]
}
```

### Specifier.nature enum

- `ImportSpecifier`
- `ImportDefaultSpecifier`
- `ImportNamespaceSpecifier`

## ThemeNameResolver

```json
{
  "server": {
    "directive": "server-only",
    "import": {
      "source": "@znode/utils/theme-resolver/theme-resolver.server",
      "specifiers": [
        { "nature": "ImportDefaultSpecifier", "value": "getThemeCookieServer" }
      ]
    },
    "variable": {
      "kind": "const",
      "declarations": [
        {
          "name": "themeName",
          "nature": "Await",
          "value": "getThemeCookieServer",
          "arguments": []
        }
      ]
    }
  },
  "client": {
    "directive": "client-only",
    "import": {
      "source": "@znode/utils/theme-resolver/theme-resolver.client",
      "specifiers": [
        { "nature": "ImportDefaultSpecifier", "value": "getThemeCookieClient" }
      ]
    },
    "variable": {
      "kind": "const",
      "declarations": [
        {
          "name": "themeName",
          "nature": "Await",
          "value": "getThemeCookieClient",
          "arguments": []
        }
      ]
    }
  }
}
```

Resolver selection depends on mapping directive.

## Variable and Declaration

### Variable

- `kind`: `var | let | const`
- `declarations`: `Declaration[]`

### Declaration.nature enum

- `Await`: emits `await value()`.
- `Call`: emits `value("arg1", ...)`.
- `Literal`: emits string literal.

## Full Working Example

The current fixture config file is a complete working example:

- `tests/config.json`

It includes two mapping suffixes:

- `signup/page.tsx`
- `home/page.tsx`

Both route through server resolver and component-level theming for base/custom1/custom2 variants.
