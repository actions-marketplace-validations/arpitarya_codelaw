---
id: py/reimplemented-starmap
title: "itertools.starmap, not a comprehension that unpacks"
languages: [python]
fix: suggest
fix_reason: "`starmap` returns an iterator; wrap it in `list()` where a list is needed"
stability: preview
ported_from: [ruff/FURB140]
since: 0.5.0
---

A generator `(f(a, b) for a, b in pairs)` is written `itertools.starmap(f, pairs)`.

**Why:** `starmap` says each item is unpacked into the call.

```codelaw
match:
  kind: generator_expression
  pattern: ($F($A, $B) for $A, $B in $P)
  not: {has: {kind: if_clause}}
constraints:
  F: {any: [{kind: identifier}, {kind: attribute}]}
message: "Write `itertools.starmap($F, $P)`"
fix: "itertools.starmap($F, $P)"
```

## Examples

### Python

#### Don't

```python
totals = (add(a, b) for a, b in pairs)
```

#### Do

```python
totals = itertools.starmap(add, pairs)
```
