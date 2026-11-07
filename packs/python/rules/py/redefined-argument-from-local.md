---
id: py/redefined-argument-from-local
title: "A loop, with or except target does not reuse a parameter"
languages: [python]
fix: manual
fix_reason: "the new name is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLR1704, pylint/redefined-argument-from-local]
since: 0.5.0
---

A `for`, `with` or `except` target inside a function is not named like one of the function's parameters.

**Why:** The parameter's value is silently replaced, and code after the block reads the wrong one.

```codelaw
match:
  kind: identifier
  pattern: $P
  any:
    - inside: {kind: for_statement, field: left}
    - inside: {kind: as_pattern_target}
  inside:
    kind: function_definition
    stopBy: {any: [{kind: function_definition}, {kind: lambda}]}
    has: {field: parameters, has: {kind: identifier, pattern: $P}}
message: "`$P` replaces the parameter of that name"
```

## How to fix

1. Rename the loop, `with` or `except` target.

## Examples

### Python

#### Don't

```python
def total(item):
    for item in items:
        pass
```

#### Do

```python
def total(item):
    for other in items:
        pass
```
