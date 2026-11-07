---
id: py/raise-vanilla-class
title: "No raise Exception"
languages: [python]
fix: manual
fix_reason: "which exception class fits is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/TRY002, pylint/broad-exception-raised]
since: 0.5.0
---

Code raises a specific exception class, its own or a builtin one, not `Exception` or `BaseException` itself.

**Why:** A caller can only catch a bare `Exception` with a broad handler that also catches every bug.

```codelaw
match:
  kind: raise_statement
  any:
    - regex: '^raise\s+(builtins\.)?(Exception|BaseException)\s*$'
    - has: {kind: call, has: {field: function, regex: '^(builtins\.)?(Exception|BaseException)$'}}
message: "Raise a specific exception class"
```

## How to fix

1. Raise a builtin exception that fits (`ValueError`, `LookupError`, …) or a class of your own derived from `Exception`.

## Examples

### Python

#### Don't

```python
raise Exception("no such user")
```

#### Do

```python
raise LookupError("no such user")
```
