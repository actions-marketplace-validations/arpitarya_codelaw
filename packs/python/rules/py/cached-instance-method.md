---
id: py/cached-instance-method
title: "No `lru_cache` on methods"
languages: [python]
fix: suggest
fix_reason: "a method that never reads `self` becomes a `@staticmethod` without it: the cache is then shared by every instance and a subclass's override no longer replaces it through the base, so review it. A method that reads `self` is manual, as the cache must move out of the class"
fix_exception: restructure
stability: preview
ported_from: [ruff/B019, pylint/method-cache-max-size-none]
since: 0.1.0
---

`functools.lru_cache` and `functools.cache` decorate functions and static methods, not instance methods.

**Why:** The cache keeps a reference to every `self` it sees, so instances are never freed.

```codelaw
native: py/cached-instance-method
```

## How to fix

1. Move the cached work into a module-level function or a `staticmethod`.
2. Or keep a per-instance cache in an attribute.

## Examples

### Python

#### Don't

```py
import functools


class Store:
    @functools.lru_cache
    def get(self, key):
        return key.upper()
```

#### Fixed

```py
import functools


class Store:
    @staticmethod
    @functools.lru_cache
    def get(key):
        return key.upper()
```

#### Do

```py
import functools


@functools.lru_cache
def normalise(key):
    return key.upper()


class Store:
    def get(self, key):
        return normalise(key)
```
