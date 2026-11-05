---
id: py/path-constructor-current-directory
title: "Path(), not Path(\".\")"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PTH201]
since: 0.5.0
---

`Path(".")` is written `Path()`.

**Why:** The current directory is the default.

```codelaw
match:
  kind: argument_list
  regex: '^\(\s*["'']\.["'']\s*\)$'
  inside: {kind: call, has: {field: function, regex: '^(pathlib\.)?(Path|PurePath)$'}}
message: "Write `Path()`"
fix: '()'
```

## Examples

### Python

#### Don't

```python
here = Path(".")
```

#### Do

```python
here = Path()
```
