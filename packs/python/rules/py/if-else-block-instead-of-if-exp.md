---
id: py/if-else-block-instead-of-if-exp
title: "A conditional expression for a two-way assignment"
languages: [python]
fix: suggest
fix_reason: "the statement form becomes an expression; review long branches"
stability: preview
ported_from: [ruff/SIM108, pylint/consider-ternary-expression]
since: 0.5.0
---

`if c: x = a else: x = b` is written `x = a if c else b`.

**Why:** One line states the one assignment.

```codelaw
match:
  kind: if_statement
  pattern: |
    if $C:
        $X = $A
    else:
        $X = $B
constraints:
  X: {any: [{kind: identifier}, {kind: attribute}]}
message: "Write `$X = $A if $C else $B`"
fix: "$X = $A if $C else $B"
```

## Examples

### Python

#### Don't

```python
if fast:
    mode = "quick"
else:
    mode = "full"
```

#### Do

```python
mode = "quick" if fast else "full"
```
