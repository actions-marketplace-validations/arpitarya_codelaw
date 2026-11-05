---
id: py/unnecessary-iterable-allocation-for-first-element
title: "next(iter(x)), not list(x)[0]"
languages: [python]
fix: suggest
fix_reason: "an empty iterable raises StopIteration instead of IndexError"
stability: preview
ported_from: [ruff/RUF015]
since: 0.5.0
---

The first element of an iterable is taken with `next(iter(x))`, not by building a list and indexing it.

**Why:** `list(x)[0]` copies every element only to read one.

```codelaw
match:
  kind: subscript
  all:
    - has: {field: value, pattern: list($X)}
    - has: {field: subscript, regex: '^0$'}
message: "Use `next(iter($X))` instead of building a list"
fix: "next(iter($X))"
```

## Examples

### Python

#### Don't

```python
first = list(names)[0]
```

#### Do

```python
first = next(iter(names))
```
