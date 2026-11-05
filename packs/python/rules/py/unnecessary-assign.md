---
id: py/unnecessary-assign
title: "Return the value, not a name assigned just before"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/RET504]
since: 0.5.0
---

`x = value` immediately followed by `return x` is written `return value`.

**Why:** The name holds the value for one line and adds nothing.

```codelaw
match:
  kind: return_statement
  pattern: return $X
  follows: {kind: expression_statement, pattern: $ASSIGN, has: {kind: assignment, pattern: $X = $V, not: {has: {field: type, kind: type}}}}
  not: {inside: {kind: function_definition, stopBy: end, has: {stopBy: end, any: [{kind: global_statement}, {kind: nonlocal_statement}]}}}
constraints:
  X: {kind: identifier}
message: "Return the value directly"
fix:
  - remove: $ASSIGN
  - replace: "return $V"
```

## Examples

### Python

#### Don't

```python
def total(xs):
    result = sum(xs)
    return result
```

#### Do

```python
def total(xs):
    return sum(xs)
```
