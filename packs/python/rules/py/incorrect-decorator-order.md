---
id: py/incorrect-decorator-order
title: "Decorators that only work in one order are written in it"
languages: [python]
fix: suggest
fix_reason: "swapping the two decorators changes what the function becomes, which is the point; the author accepts it"
stability: preview
ported_from: [ruff/RUF074]
since: 0.5.0
---

Two adjacent decorators are in an order that breaks at run time or does the wrong thing silently. Decorators apply bottom-up, so the outer one wraps what the inner one returns:

- `@abstractmethod` goes below `@property`, `@classmethod`, `@staticmethod` and a property's `.setter`, `.getter` or `.deleter`;
- `@contextmanager` and `@asynccontextmanager` go below `@classmethod`;
- `@cache` and `@lru_cache` go below `@property`, `@classmethod` and `@cached_property`;
- `@cached_property` goes below `@abstractmethod`;
- `@classmethod` above `@cached_property` stopped working in Python 3.13, and is reported.

Names are read as written: `abstractmethod` or `abc.abstractmethod`, `lru_cache` or `functools.lru_cache`, with or without call arguments. A decorator between the two breaks the pair, as upstream.

**Why:** In the wrong order the method is not abstract, the cache is shared across instances, or Python raises when the class is created.

```codelaw
utils:
  abstract: {kind: decorator, regex: '^@\s*(abc\.)?abstractmethod\s*$'}
  propertyish: {kind: decorator, regex: '^@\s*((builtins\.)?property|[\w.]+\.(setter|getter|deleter))\s*$'}
  classmethod: {kind: decorator, regex: '^@\s*(builtins\.)?classmethod\s*$'}
  staticmethod: {kind: decorator, regex: '^@\s*(builtins\.)?staticmethod\s*$'}
  contextmanager: {kind: decorator, regex: '^@\s*(contextlib\.)?(async)?contextmanager\s*$'}
  cache: {kind: decorator, regex: '^@\s*(functools\.)?(cache|lru_cache)\s*(\(.*\))?\s*$'}
  cachedproperty: {kind: decorator, regex: '^@\s*(functools\.)?cached_property\s*$'}
match:
  kind: decorator
  any:
    - all:
        - matches: abstract
        - precedes: {stopBy: neighbor, pattern: $B, any: [{matches: propertyish}, {matches: classmethod}, {matches: staticmethod}]}
    - all:
        - matches: contextmanager
        - precedes: {stopBy: neighbor, pattern: $B, matches: classmethod}
    - all:
        - matches: cache
        - precedes: {stopBy: neighbor, pattern: $B, any: [{matches: propertyish}, {matches: classmethod}, {matches: cachedproperty}]}
    - all:
        - matches: cachedproperty
        - precedes: {stopBy: neighbor, pattern: $B, matches: abstract}
    - all:
        - matches: classmethod
        - precedes: {stopBy: neighbor, pattern: $B, matches: cachedproperty}
message: "Put `$$MATCH` below `$B`"
fix:
  - replace: { target: $B, with: "$$MATCH" }
  - replace: "$B"
```

## Examples

### Python

#### Don't

```python
from abc import abstractmethod


class Shape:
    @abstractmethod
    @property
    def area(self): ...
```

#### Do

```python
from abc import abstractmethod


class Shape:
    @property
    @abstractmethod
    def area(self): ...
```
