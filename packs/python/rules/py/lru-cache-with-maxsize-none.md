---
id: py/lru-cache-with-maxsize-none
title: "functools.cache, not lru_cache(maxsize=None)"
languages: [python]
fix: auto
fix_target: { python: "3.9" }
stability: preview
ported_from: [ruff/UP033]
since: 0.5.0
---

`@functools.lru_cache(maxsize=None)` is written `@functools.cache` (Python 3.9+).

**Why:** `cache` is the unbounded cache under its own name.

```codelaw
match:
  kind: call
  pattern: functools.lru_cache(maxsize=None)
  inside: {kind: decorator}
message: "Write `@functools.cache`"
fix: "functools.cache"
```

## Examples

### Python

#### Don't

```python
@functools.lru_cache(maxsize=None)
def f():
    pass
```

#### Do

```python
@functools.cache
def f():
    pass
```
