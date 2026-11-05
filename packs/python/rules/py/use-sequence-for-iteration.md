---
id: py/use-sequence-for-iteration
title: "Iterate a tuple, not a set literal"
languages: [python]
fix: suggest
fix_reason: "the order becomes the written order, where the set's was arbitrary; duplicates are kept"
stability: preview
ported_from: [ruff/PLC0208, pylint/use-sequence-for-iteration]
since: 0.5.0
---

A `for` loop does not iterate a set literal.

**Why:** A set's order is arbitrary, so the loop's order is too.

```codelaw
match:
  kind: set
  pattern: "{$A, $$$R}"
  inside: {kind: for_statement, field: right}
message: "Iterate a tuple; a set's order is arbitrary"
fix: "($A, $$$R)"
```

## Examples

### Python

#### Don't

```python
for mode in {"r", "w"}:
    print(mode)
```

#### Do

```python
for mode in ("r", "w"):
    print(mode)
```
