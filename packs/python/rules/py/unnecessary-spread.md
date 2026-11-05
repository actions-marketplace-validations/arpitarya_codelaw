---
id: py/unnecessary-spread
title: "No spread of a dict literal"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PIE800]
since: 0.5.0
---

`{**a, **{"b": 1}}` is written `{**a, "b": 1}` (a spread literal of one entry; Ruff also inlines longer ones).

**Why:** The inner literal's entries can be written in place.

```codelaw
match:
  kind: dictionary_splat
  has: {kind: dictionary, has: {kind: pair, pattern: $P}, not: {has: {any: [{kind: dictionary_splat}, {kind: pair, nthChild: 2}, {kind: comment}]}}}
  inside: {kind: dictionary}
message: "Write the entries in place"
fix: "$P"
```

## Examples

### Python

#### Don't

```python
opts = {**base, **{"debug": True}}
```

#### Do

```python
opts = {**base, "debug": True}
```
