---
id: py/unnecessary-key-check
title: "d.get(k), not k in d and d[k]"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/RUF019]
since: 0.5.0
---

`key in d and d[key]` is written `d.get(key)`.

**Why:** One lookup instead of two, and the intent reads directly.

```codelaw
match:
  pattern: $K in $D and $D[$K]
constraints:
  K: {any: [{kind: identifier}, {kind: string}, {kind: integer}]}
  D: {any: [{kind: identifier}, {kind: attribute}]}
message: "Use `$D.get($K)`"
fix: "$D.get($K)"
```

## Examples

### Python

#### Don't

```python
if "debug" in opts and opts["debug"]:
    trace()
```

#### Do

```python
if opts.get("debug"):
    trace()
```
