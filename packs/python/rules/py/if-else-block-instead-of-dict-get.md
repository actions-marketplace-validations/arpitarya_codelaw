---
id: py/if-else-block-instead-of-dict-get
title: "d.get(k, default), not if k in d"
languages: [python]
fix: suggest
fix_reason: "`get` evaluates the default even when the key is present"
stability: preview
ported_from: [ruff/SIM401, pylint/consider-using-get]
since: 0.5.0
---

`if k in d: v = d[k] else: v = default` is written `v = d.get(k, default)`.

**Why:** `get` is the one lookup the branches spell out.

```codelaw
match:
  kind: if_statement
  pattern: |
    if $K in $D:
        $V = $D[$K]
    else:
        $V = $E
message: "Write `$V = $D.get($K, $E)`"
fix: "$V = $D.get($K, $E)"
```

## Examples

### Python

#### Don't

```python
if "port" in cfg:
    port = cfg["port"]
else:
    port = 80
```

#### Do

```python
port = cfg.get("port", 80)
```
