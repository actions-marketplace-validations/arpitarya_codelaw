---
id: py/os-path-commonprefix
title: "os.path.commonpath, not commonprefix"
languages: [python]
fix: suggest
fix_reason: "`commonpath` compares whole path components and raises on mixed absolute and relative paths"
stability: preview
ported_from: [ruff/RUF071]
since: 0.5.0
---

The common directory of paths is `os.path.commonpath`, not `os.path.commonprefix`.

**Why:** `commonprefix` compares characters: the common prefix of `/usr/lib` and `/usr/local` is `/usr/l`.

```codelaw
match:
  pattern: os.path.commonprefix($$$A)
message: "`commonprefix` compares characters; use `os.path.commonpath`"
fix: "os.path.commonpath($$$A)"
```

## Examples

### Python

#### Don't

```python
root = os.path.commonprefix(paths)
```

#### Do

```python
root = os.path.commonpath(paths)
```
