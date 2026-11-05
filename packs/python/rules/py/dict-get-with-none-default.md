---
id: py/dict-get-with-none-default
title: "d.get(k), not d.get(k, None)"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/SIM910]
since: 0.5.0
---

`d.get(k, None)` is written `d.get(k)`.

**Why:** `None` is already the default.

```codelaw
match:
  pattern: $D.get($K, None)
constraints:
  K: {not: {any: [{kind: keyword_argument}, {kind: list_splat}]}}
message: "`None` is the default; write `$D.get($K)`"
fix: "$D.get($K)"
```

## Examples

### Python

#### Don't

```python
port = cfg.get("port", None)
```

#### Do

```python
port = cfg.get("port")
```
