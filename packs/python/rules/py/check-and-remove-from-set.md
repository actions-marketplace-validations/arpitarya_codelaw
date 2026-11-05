---
id: py/check-and-remove-from-set
title: "s.discard(x), not if x in s: s.remove(x)"
languages: [python]
fix: suggest
fix_reason: "the same for a set; another type may lack `discard`"
stability: preview
ported_from: [ruff/FURB132]
since: 0.5.0
---

`if x in s: s.remove(x)` is written `s.discard(x)`.

**Why:** One call does the check and the removal.

```codelaw
match:
  kind: if_statement
  pattern: |
    if $X in $S:
        $S.remove($X)
  not: {has: {any: [{kind: else_clause}, {kind: elif_clause}]}}
message: "Write `$S.discard($X)`"
fix: "$S.discard($X)"
```

## Examples

### Python

#### Don't

```python
if name in seen:
    seen.remove(name)
```

#### Do

```python
seen.discard(name)
```
