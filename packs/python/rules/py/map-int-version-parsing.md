---
id: py/map-int-version-parsing
title: "No map(int, version.split('.'))"
languages: [python]
fix: manual
fix_reason: "the right parser (packaging.version, or a tuple the library exposes) is the author's"
stability: preview
ported_from: [ruff/RUF048]
since: 0.5.0
---

A version string is not parsed with `map(int, v.split("."))`.

**Why:** Versions like `1.2.0rc1` or `2.0.post1` raise ValueError; a version parser handles them.

```codelaw
match:
  pattern: map(int, $V.split("."))
constraints:
  V: {regex: '(?i)version'}
message: "Parse the version with `packaging.version.Version`"
```

## How to fix

1. Use `packaging.version.Version(v)`, or a tuple attribute the library already exposes (`version_info`).

## Examples

### Python

#### Don't

```python
parts = tuple(map(int, lib.__version__.split(".")))
```

#### Do

```python
parts = lib.version_info
```
