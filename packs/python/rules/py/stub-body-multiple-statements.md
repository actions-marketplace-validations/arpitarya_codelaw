---
id: py/stub-body-multiple-statements
title: "A stub function's body is one statement"
languages: [python]
files: ["**/*.pyi"]
fix: manual
fix_reason: "the body is replaced by `...`, and a docstring kept if wanted"
fix_exception: restructure
stability: preview
ported_from: [ruff/PYI048]
since: 0.5.0
---

A function in a stub has a body of one statement: `...`, or a docstring.

**Why:** A stub declares; any code in it is never run, and several statements suggest it was copied from an implementation.

```codelaw
match:
  kind: function_definition
  has: {field: body, has: {nthChild: 2, not: {kind: comment}}}
message: "A stub function's body is `...`"
```

## How to fix

1. Replace the body with `...` (a docstring may stay).

## Examples

### Python

#### Don't

```python path=api.pyi
def f() -> int:
    x = 1
    return x
```

#### Do

```python path=api.pyi
def f() -> int: ...
```
