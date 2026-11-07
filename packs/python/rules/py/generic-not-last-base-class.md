---
id: py/generic-not-last-base-class
title: "`Generic[...]` is the last base class"
languages: [python]
fix: suggest
fix_reason: "moving the base changes the class's method resolution order, which is what the rule asks for; a comment on the moved base may need moving by hand"
fix_exception: restructure
stability: preview
ported_from: [ruff/PYI059]
since: 0.5.0
---

`Generic` (or `Generic[T]`, from `typing` or `typing_extensions`) is listed before another positional base class. Keyword arguments such as `metaclass=` do not count. The fix moves it after the last positional base; a class with a `*bases` argument, or with `Generic` twice, is reported without a fix.

**Why:** Python builds the method resolution order from the bases in order, and `Generic` placed first can raise `TypeError` when the class is created, or put `Generic` ahead of the class's real bases.

```codelaw
utils:
  generic: {regex: '^((typing|typing_extensions|t)\.)?Generic(\s*\[[\s\S]*\])?$', any: [{kind: identifier}, {kind: attribute}, {kind: subscript}]}
  positional: {any: [{kind: identifier}, {kind: attribute}, {kind: subscript}, {kind: call}]}
match:
  kind: argument_list
  inside: {kind: class_definition, field: superclasses}
  has:
    matches: generic
    precedes: {stopBy: end, matches: positional}
  any:
    - all:
        - not: {has: {kind: list_splat}}
        - not: {has: {matches: generic, precedes: {stopBy: end, matches: generic}}}
        - has: {matches: generic, pattern: $G}
        - has:
            matches: positional
            pattern: $LAST
            not: {precedes: {stopBy: end, matches: positional}}
    - has: {kind: list_splat}
    - has: {matches: generic, precedes: {stopBy: end, matches: generic}}
message: "Put `Generic` after the other base classes"
fix:
  - remove: $G
  - insert-after: { target: $LAST, text: ", $G" }
```

## Examples

### Python

#### Don't

```python
from typing import Generic, Sized, TypeVar

T = TypeVar("T")


class Box(Generic[T], Sized):
    pass
```

#### Do

```python
from typing import Generic, Sized, TypeVar

T = TypeVar("T")


class Box(Sized, Generic[T]):
    pass
```
