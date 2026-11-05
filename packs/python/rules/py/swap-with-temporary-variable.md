---
id: py/swap-with-temporary-variable
title: "a, b = b, a, not a temporary"
languages: [python]
fix: suggest
fix_reason: "the temporary name is no longer bound afterwards"
stability: preview
ported_from: [ruff/PLR1712, pylint/consider-swap-variables]
since: 0.5.0
---

`t = a; a = b; b = t` is written `a, b = b, a`.

**Why:** Tuple assignment swaps in one statement.

```codelaw
match:
  kind: expression_statement
  has: {kind: assignment, pattern: $T = $A}
  precedes:
    kind: expression_statement
    has: {kind: assignment, pattern: $A = $B}
    pattern: $S2
    precedes: {kind: expression_statement, has: {kind: assignment, pattern: $B = $T}, pattern: $S3}
constraints:
  T: {kind: identifier}
  A: {kind: identifier}
  B: {kind: identifier}
message: "Swap with `$A, $B = $B, $A`"
fix:
  - remove: $S2
  - remove: $S3
  - replace: "$A, $B = $B, $A"
```

## Examples

### Python

#### Don't

```python
tmp = x
x = y
y = tmp
```

#### Do

```python
x, y = y, x
```
