---
id: py/non-octal-permissions
title: "File modes are octal"
languages: [python]
fix: suggest
fix_reason: "the decimal literal is rewritten as the octal one it was probably meant to be; review that it was"
stability: preview
ported_from: [ruff/RUF064]
since: 0.5.0
---

A file mode passed to `os.chmod`, `os.mkdir`, `os.makedirs`, `os.open`, `Path.chmod` or `Path.mkdir` is an octal literal (`0o644`), not a decimal one (`644`).

**Why:** `644` in decimal is `0o1204`: a different, usually wrong, set of permissions.

```codelaw
match:
  kind: integer
  regex: '^[0-7]{3,4}$'
  pattern: $N
  inside:
    kind: argument_list
    inside: {kind: call, has: {field: function, regex: '^(os\.(chmod|mkdir|makedirs|fchmod|lchmod)|\w+\.chmod)$'}}
message: "`$N` is decimal; write `0o$N`"
fix: "0o$N"
```

## Examples

### Python

#### Don't

```python
os.chmod(path, 644)
```

#### Do

```python
os.chmod(path, 0o644)
```
