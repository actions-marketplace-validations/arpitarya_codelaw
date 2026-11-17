---
id: py/too-many-ancestors
title: "A class must not have more than seven ancestors"
languages: [python]
scope: project
fix: manual
fix_reason: "flattening a deep hierarchy moves behaviour between classes, which is a change of structure"
fix_exception: restructure
stability: preview
ported_from: [pylint/too-many-ancestors]
since: 0.5.0
---

A class with more ancestors than `[graph] max_ancestors` (default 7, Pylint's `max-parents`) is reported at its name, with the count: `Too many ancestors (9/7)`. Ancestors are every distinct class its bases lead to, through the project's files — a base named through `from m import C`, `import m` and `m.C`, a re-export, or a class in the same file — each counted once. A base from outside the project (the standard library, an installed package) counts as one itself and not for its own ancestors, which the project does not hold, so the count can fall short of Pylint's and never exceeds it; `object`, `tuple`, `dict`, `list` and `set` are not counted. A subscript (`Generic[T]`) names its class; a keyword (`metaclass=`), a call and `*bases` are not bases.

**Why:** a class that inherits from a deep hierarchy is hard to read: where a method comes from, and what a change to any ancestor breaks, is spread over many files.

```codelaw
graph:
  python:
    classes: ancestors
message: "Too many ancestors ($COUNT/$MAX)"
```

## How to fix

1. Prefer composition: hold the behaviour as an attribute instead of inheriting it.
2. Or merge layers of the hierarchy that add little.

## Examples

### Python

#### Don't

```py files
# app/__init__.py
# app/base.py
class A: ...
class B(A): ...
class C(B): ...
class D(C): ...
class E(D): ...
class F(E): ...
class G(F): ...
class H(G): ...
# app/leaf.py
from app.base import H


class Leaf(H): ...
```

#### Do

```py files
# app/__init__.py
# app/base.py
class A: ...
class B(A): ...
# app/leaf.py
from app.base import B


class Leaf(B): ...
```
