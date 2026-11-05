---
id: py/implicit-cwd
title: "Path.cwd(), not Path().resolve()"
languages: [python]
fix: suggest
fix_reason: "`cwd` does not resolve symlinks in the current directory's path"
stability: preview
ported_from: [ruff/FURB177]
since: 0.5.0
---

`Path().resolve()` and `Path(".").resolve()` are written `Path.cwd()`.

**Why:** `cwd` names what is meant.

```codelaw
match:
  kind: call
  all:
  - has: {field: function, kind: attribute, regex: '^(pathlib\.)?Path\(\s*(["'']\.["''])?\s*\)\.resolve$'}
  - has: {field: arguments, regex: ^\(\s*\)$}
message: Write `Path.cwd()`
fix: Path.cwd()
```

## Examples

### Python

#### Don't

```python
here = Path().resolve()
```

#### Do

```python
here = Path.cwd()
```
