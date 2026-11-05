---
id: py/if-key-in-dict-del
title: "d.pop(k, None), not if k in d: del d[k]"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/RUF051]
since: 0.5.0
---

`if key in d: del d[key]` is written `d.pop(key, None)`.

**Why:** One call, one lookup, and no branch.

```codelaw
match:
  kind: if_statement
  pattern: |
    if $K in $D:
        del $D[$K]
  not: {has: {any: [{kind: else_clause}, {kind: elif_clause}]}}
constraints:
  D: {any: [{kind: identifier}, {kind: attribute}]}
message: "Use `$D.pop($K, None)`"
fix: "$D.pop($K, None)"
```

## Examples

### Python

#### Don't

```python
if key in cache:
    del cache[key]
```

#### Do

```python
cache.pop(key, None)
```
