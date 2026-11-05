---
id: py/misplaced-bare-raise
title: "No bare raise outside an except block"
languages: [python]
fix: manual
fix_reason: "which exception to raise is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PLE0704]
since: 0.5.0
---

A bare `raise` appears only inside an `except` block (or a function called from one).

**Why:** Elsewhere there is no exception to re-raise, and it raises RuntimeError.

```codelaw
match:
  kind: raise_statement
  regex: '^raise$'
  not: {inside: {stopBy: end, any: [{kind: except_clause}, {kind: function_definition}]}}
message: "There is no active exception to re-raise here"
```

## How to fix

1. Raise a specific exception: `raise ValueError(...)`.

## Examples

### Python

#### Don't

```python
if bad:
    raise
```

#### Do

```python
if bad:
    raise ValueError("bad input")
```
