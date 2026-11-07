---
id: py/assignment-in-assert
title: "No walrus inside assert"
languages: [python]
fix: suggest
fix_reason: "the value is now computed even when asserts are stripped (`python -O`), which is the point but runs its code there too; a binding that is not the first thing the assert evaluates is moved by hand"
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
  any:
    - all:
        - has: { field: name, pattern: $N }
        - has: { field: value, pattern: $V }
        - inside:
            kind: parenthesized_expression
            nthChild: 1
            pattern: $PAR
            any:
              - inside: { kind: assert_statement, pattern: $A }
              - inside: { kind: comparison_operator, nthChild: 1, inside: { kind: assert_statement, pattern: $A } }
    - regex: ''
message: "This name is unbound when asserts are stripped (`python -O`)"
fix:
  - insert-before: { target: $A, text: "$N = $V", line: true }
  - replace: { target: $PAR, with: "$N" }
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
