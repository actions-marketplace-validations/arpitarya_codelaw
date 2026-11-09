---
id: py/signature-in-docstring
title: "A docstring's first line is not the function's signature"
languages: [python]
fix: manual
fix_reason: "a summary in words is the author's"
fix_exception: authored
stability: preview
ported_from: [ruff/D402]
since: 0.5.0
---

The first line of a function's docstring does not contain a call-shaped repeat of the function's own name, such as `f(a, b) -> int`.

**Why:** The signature is already in the code, and tools show it beside the docstring; the first line is the place for the summary.

```codelaw
native: py/signature-in-docstring
```

## How to fix

1. Replace the first line with one sentence saying what the function does.

## Examples

### Python

#### Don't

```python
def add(a, b):
    """add(a, b) -> int"""
```

#### Do

```python
def add(a, b):
    """Return the sum of a and b."""
```
