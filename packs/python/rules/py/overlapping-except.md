---
id: py/overlapping-except
title: "An except tuple does not list an exception twice over"
languages: [python]
fix: manual
fix_reason: "whether the broad or the narrow name was meant is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/overlapping-except]
since: 0.5.0
---

An `except (A, B)` tuple does not list `Exception` (or `BaseException`) beside another exception it already covers.

**Why:** The narrower name is dead; the tuple reads as if it were handled separately.

```codelaw
match:
  kind: tuple
  inside: {kind: except_clause}
  all:
    - has: {kind: identifier, regex: '^(Exception|BaseException)$'}
    - has: {kind: identifier, regex: '(Error|Exception|Warning|Exit|Interrupt|Iteration)$', not: {regex: '^(Exception|BaseException)$'}}
message: "`Exception` already covers the others in the tuple"
```

## How to fix

1. Keep the exceptions the handler is meant for, and drop the ones another already covers.

## Examples

### Python

#### Don't

```python
try:
    run()
except (ValueError, Exception):
    pass
```

#### Do

```python
try:
    run()
except Exception:
    pass
```
