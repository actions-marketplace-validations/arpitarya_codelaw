---
id: py/reimplemented-container-builtin
title: "list, not lambda: []"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PIE807]
since: 0.5.0
---

`lambda: []` is written `list` (Ruff also rewrites `lambda: {}` to `dict`; that half is left out).

**Why:** `list` is the factory the lambda spells out.

```codelaw
match:
  kind: lambda
  regex: '^lambda\s*:\s*\[\s*\]$'
message: "Use `list`"
fix: "list"
```

## Examples

### Python

#### Don't

```python
groups = defaultdict(lambda: [])
```

#### Do

```python
groups = defaultdict(list)
```
