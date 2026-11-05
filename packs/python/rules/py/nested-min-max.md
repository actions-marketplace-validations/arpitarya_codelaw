---
id: py/nested-min-max
title: "One min() or max(), not nested"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PLW3301]
since: 0.5.0
---

`min(a, min(b, c))` is written `min(a, b, c)` (and the same for `max`).

**Why:** One call takes all the arguments.

```codelaw
match:
  pattern: $F($A, $F($B, $C))
constraints:
  F: {regex: '^(min|max)$'}
  A: {not: {any: [{kind: keyword_argument}, {kind: list_splat}]}}
  B: {not: {any: [{kind: keyword_argument}, {kind: list_splat}]}}
  C: {not: {any: [{kind: keyword_argument}, {kind: list_splat}]}}
message: "Merge into one `$F($A, $B, $C)`"
fix: "$F($A, $B, $C)"
```

## Examples

### Python

#### Don't

```python
low = min(a, min(b, c))
```

#### Do

```python
low = min(a, b, c)
```
