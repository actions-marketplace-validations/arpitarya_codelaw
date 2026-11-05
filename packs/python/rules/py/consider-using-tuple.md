---
id: py/consider-using-tuple
title: "Iterate a tuple literal, not a list literal"
languages: [python]
fix: auto
stability: preview
ported_from: [pylint/consider-using-tuple]
since: 0.5.0
---

A `for` loop over a list literal iterates a tuple literal instead.

**Why:** The tuple is a constant; the list is built on every run.

```codelaw
match:
  kind: list
  pattern: "[$A, $$$R]"
  not: {has: {kind: list_splat}}
  inside: {kind: for_statement, field: right}
message: "Iterate a tuple"
fix: "($A, $$$R)"
```

## Examples

### Python

#### Don't

```python
for mode in ["r", "w"]:
    print(mode)
```

#### Do

```python
for mode in ("r", "w"):
    print(mode)
```
