---
id: py/zip-dict-keys-and-values
title: "d.items(), not zip(d.keys(), d.values())"
languages: [python]
fix: suggest
fix_reason: "the same for a dict; a mapping whose views differ would change"
stability: preview
ported_from: [ruff/SIM911]
since: 0.5.0
---

`zip(d.keys(), d.values())` is written `d.items()`.

**Why:** `items` gives the pairs directly.

```codelaw
match:
  pattern: zip($D.keys(), $D.values())
message: "Write `$D.items()`"
fix: "$D.items()"
```

## Examples

### Python

#### Don't

```python
for k, v in zip(cfg.keys(), cfg.values()):
    pass
```

#### Do

```python
for k, v in cfg.items():
    pass
```
