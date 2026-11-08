---
id: py/non-self-return-type
title: "A method that returns self is annotated Self"
languages: [python]
fix: suggest
fix_reason: "`Self` follows a subclass, where the class name did not, so a type checker reads callers differently; `Self` is in `typing` from Python 3.11 (`typing_extensions` before), so the import is reviewed on older targets"
stability: preview
ported_from: [ruff/PYI034]
since: 0.5.0
---

A method that by convention returns its instance is annotated `Self`, not the class's own name: `__new__`, `__enter__` and `async def __aenter__`, unless the class is decorated `@final`; the in-place operators (`__iadd__`, `__isub__`, … `__ior__`); and `__iter__` returning `Iterator` (or `Iterable`) in a class whose bases name `Iterator`, `__aiter__` likewise with `AsyncIterator`. The fix writes `Self`, drops a first parameter's annotation that names the class (`self: Shape`), and imports `Self` from `typing`. A metaclass (a class whose bases name `type`), an `@abstractmethod`, an `@overload` and a method with no positional parameter are not reported; nor is a quoted class name.

**Why:** `Shape.__enter__` annotated `-> Shape` tells a type checker that `with Circle() as c` gives a `Shape`; `Self` gives the `Circle` the method returns.

```codelaw
match:
  kind: function_definition
  all:
    - inside:
        any:
          - {kind: block, inside: {kind: class_definition, has: {field: name, pattern: $C}, not: {inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@(typing\.)?final$'}}}}}
          - {kind: decorated_definition, not: {has: {kind: decorator, regex: '^@(abc\.)?abstractmethod$|^@(typing\.)?overload$'}}, inside: {kind: block, inside: {kind: class_definition, has: {field: name, pattern: $C}, not: {inside: {kind: decorated_definition, has: {kind: decorator, regex: '^@(typing\.)?final$'}}}}}}
    - not: {inside: {kind: block, inside: {kind: class_definition, has: {field: superclasses, has: {regex: '^(builtins\.)?type$|^(abc\.)?ABCMeta$|^type\['}}}}}
    - not: {inside: {kind: decorated_definition, inside: {kind: block, inside: {kind: class_definition, has: {field: superclasses, has: {regex: '^(builtins\.)?type$|^(abc\.)?ABCMeta$'}}}}}}
    - has: {field: parameters, has: {nthChild: 1, any: [{kind: identifier}, {kind: typed_parameter}, {kind: default_parameter}, {kind: typed_default_parameter}]}}
    - has: {field: return_type, pattern: $R}
    - any:
        - all:
            - has: {field: name, regex: '^(__new__|__enter__|__i(add|sub|mul|matmul|truediv|floordiv|mod|pow|lshift|rshift|and|xor|or)__)$'}
            - not: {regex: '^async\b'}
            - has: {field: return_type, pattern: $C}
        - all:
            - has: {field: name, regex: '^__aenter__$'}
            - regex: '^async\b'
            - has: {field: return_type, pattern: $C}
        - all:
            - has: {field: name, regex: '^__iter__$'}
            - has: {field: return_type, regex: '^((typing|collections\.abc)\.)?(Iterator|Iterable)\b'}
            - inside: {kind: block, inside: {kind: class_definition, has: {field: superclasses, has: {regex: '^((typing|collections\.abc)\.)?Iterator\b'}}}}
        - all:
            - has: {field: name, regex: '^__aiter__$'}
            - has: {field: return_type, regex: '^((typing|collections\.abc)\.)?(AsyncIterator|AsyncIterable)\b'}
            - inside: {kind: block, inside: {kind: class_definition, has: {field: superclasses, has: {regex: '^((typing|collections\.abc)\.)?AsyncIterator\b'}}}}
    - any:
        - has: {field: parameters, has: {nthChild: 1, kind: typed_parameter, all: [{has: {kind: identifier, pattern: $SELF}}, {has: {field: type, pattern: $C}}], pattern: $P}}
        - regex: ''
    - has: {field: name, pattern: $M}
message: "`$M` returns the instance: annotate it `Self`"
fix:
  - replace: { target: $R, with: "Self" }
  - replace: { target: $P, with: "$SELF" }
  - ensure-import: { module: typing, name: Self }
```

## How to fix

1. Annotate the return `Self`, imported from `typing` (`typing_extensions` before Python 3.11), and drop the class name from the first parameter's annotation.

## Examples

### Python

#### Don't

```python
class Session:
    def __enter__(self) -> Session:
        return self
```

#### Do

```python
from typing import Self

class Session:
    def __enter__(self) -> Self:
        return self
```
