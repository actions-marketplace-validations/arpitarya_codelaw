---
id: py/undefined-loop-variable
title: "A loop variable is read only where the loop ran"
languages: [python]
fix: manual
fix_reason: "what to do for an empty iterable is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/undefined-loop-variable]
since: 0.5.0
---

A `for` target is read after the loop, and the loop may not have run: its iterable is not a non-empty literal, a `range` of integer literals, or a builtin (`enumerate`, `zip`, `list`, ...) over one. An `else:` that rebinds the name or leaves the function, a binding before the loop, and a read inside the loop are fine.

**Why:** With an empty iterable the name is unbound (or still holds an older value) at the read.

```codelaw
native: py/undefined-loop-variable
```

## How to fix

1. Bind the name before the loop, or
2. handle the empty iterable explicitly, or
3. use the value inside the loop.

## Examples

### Python

#### Don't

```python
def last(items):
    for item in items:
        pass
    return item
```

#### Do

```python
def last(items):
    item = None
    for item in items:
        pass
    return item
```
