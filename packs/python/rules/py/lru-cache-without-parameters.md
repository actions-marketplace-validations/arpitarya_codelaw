---
id: py/lru-cache-without-parameters
title: "@lru_cache, not @lru_cache()"
languages: [python]
fix: auto
fix_target: { python: "3.8" }
stability: preview
ported_from: [ruff/UP011]
since: 0.5.0
---

`@functools.lru_cache()` with no arguments is written `@functools.lru_cache`.

**Why:** Since Python 3.8 the decorator takes the function directly.

```codelaw
match:
  kind: call
  all:
  - has: {field: function, regex: '^(functools\.)?lru_cache$', pattern: $F}
  - has: {field: arguments, regex: ^\(\s*\)$}
  - inside: {kind: decorator}
message: Drop the empty parentheses
fix: $F
```

## Examples

### Python

#### Don't

```python
@functools.lru_cache()
def f():
    pass
```

#### Do

```python
@functools.lru_cache
def f():
    pass
```
