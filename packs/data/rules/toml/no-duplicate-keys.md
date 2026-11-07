---
id: toml/no-duplicate-keys
title: "No key or table defined twice in TOML"
languages: [toml]
fix: auto
fix_reason: "auto when the key is set twice to the same value, so the later one goes; otherwise the choices keep the first or the last, since TOML refuses both and only the author knows which was meant. A table defined twice is manual"
fix_exception: value
stability: preview
since: 0.1.0
---

A key is defined once in its table, and a `[table]` header appears once. That covers dotted keys (`a.b = 1` and then `a = 2`), a table defined both by a header and by dotted keys, and keys inside inline tables.

**Why:** TOML forbids defining a key or table twice and conforming parsers refuse the file, so the mistake breaks every tool that reads it.

```codelaw
native: toml/no-duplicate-keys
```

## How to fix

1. Decide which definition is right.
2. Delete the other one, or merge the two tables into one.

## Examples

### TOML

#### Don't

```toml
[tool.pytest]
addopts = "-q"
testpaths = ["tests"]
addopts = "-q"
```

#### Fixed

```toml
[tool.pytest]
addopts = "-q"
testpaths = ["tests"]
```

#### Do

```toml
[tool.pytest]
addopts = "-ra"
testpaths = ["tests"]
```
