---
id: py/assignment-in-assert
title: "No walrus inside assert"
languages: [python]
fix: manual
fix_reason: "the binding moves to its own statement before the assert"
stability: preview
ported_from: [ruff/RUF018]
since: 0.5.0
---

An `assert` does not bind a name with `:=`.

**Why:** Asserts are stripped under `python -O`, so the name would be unbound in optimised runs.

```codelaw
match:
  kind: named_expression
  inside: {kind: assert_statement, stopBy: end}
message: "This name is unbound when asserts are stripped (`python -O`)"
```

## How to fix

1. Bind the name on its own line before the assert, then assert on the name.

## Examples

### Python

#### Don't

```python
assert (n := len(items)) > 0
```

#### Do

```python
n = len(items)
assert n > 0
```
