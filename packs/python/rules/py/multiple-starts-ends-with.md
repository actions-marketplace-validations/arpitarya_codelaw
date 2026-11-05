---
id: py/multiple-starts-ends-with
title: "One startswith with a tuple"
languages: [python]
fix: auto
fix_reason: "auto when the receiver is a plain name, so evaluating it once changes nothing"
stability: preview
ported_from: [ruff/PIE810]
since: 0.5.0
---

`s.startswith(a) or s.startswith(b)` is written `s.startswith((a, b))` (and the same for `endswith`).

**Why:** One call takes the tuple of prefixes.

```codelaw
match:
  pattern: $S.$M($A) or $S.$M($B)
constraints:
  S: {kind: identifier}
  M: {regex: '^(startswith|endswith)$'}
  A: {not: {kind: tuple}}
  B: {not: {kind: tuple}}
message: "Merge into `$S.$M(($A, $B))`"
fix: "$S.$M(($A, $B))"
```

## Examples

### Python

#### Don't

```python
if name.startswith("a") or name.startswith("b"):
    pass
```

#### Do

```python
if name.startswith(("a", "b")):
    pass
```
