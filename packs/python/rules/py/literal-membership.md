---
id: py/literal-membership
title: "Test membership in a set literal"
languages: [python]
fix: suggest
fix_reason: "a set needs hashable items, which a list did not"
stability: preview
ported_from: [ruff/PLR6201, pylint/use-set-for-membership]
since: 0.5.0
---

`x in [a, b, c]` and `x in (a, b, c)` with literal items are written `x in {a, b, c}`. The fix covers lists.

**Why:** A set tests membership in constant time and says order does not matter.

```codelaw
match:
  kind: list
  pattern: "[$A, $$$R]"
  not: {has: {any: [{kind: list}, {kind: dictionary}, {kind: set}, {kind: list_splat}]}}
  inside: {kind: comparison_operator, regex: '\sin\s+\[', has: {field: operators, regex: 'in'}}
message: "Test membership in a set literal"
fix: "{$A, $$$R}"
```

## Examples

### Python

#### Don't

```python
if mode in ["a", "b"]:
    pass
```

#### Do

```python
if mode in {"a", "b"}:
    pass
```
