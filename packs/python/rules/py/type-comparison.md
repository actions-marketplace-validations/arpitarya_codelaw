---
id: py/type-comparison
title: "Don't compare types with `==`"
languages: [python]
fix: auto
fix_reason: "auto when both sides are `type()` calls or builtin type names not rebound: `==` becomes `is` and `!=` `is not`, which give the same answer there; for any other class it is a suggestion, as a metaclass may define `__eq__`. `isinstance()` also accepts subclasses, so it is not offered"
stability: preview
ported_from: [ruff/E721, pylint/unidiomatic-typecheck]
since: 0.1.0
---

Types are compared with `is` / `is not`, or checked with `isinstance()`, not with `==` or `!=` against `type(x)`.

**Why:** `type(x) == T` ignores subclasses and can be fooled by a metaclass's `__eq__`.

```codelaw
native: py/type-comparison
```

## How to fix

1. Use `isinstance(x, T)` when subclasses should match.
2. Use `type(x) is T` when only the exact type should.

## Examples

### Python

#### Don't

```py
if type(value) == int:
    count(value)
```

#### Fixed

```py
if type(value) is int:
    count(value)
```

#### Do

```py
if isinstance(value, int):
    count(value)
```
