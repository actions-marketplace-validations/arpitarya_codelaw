---
id: py/non-iterator-returned
title: "`__iter__` returns an iterator"
languages: [python]
fix: manual
fix_reason: "which iterator to return is the author's"
fix_exception: restructure
stability: preview
ported_from: [pylint/non-iterator-returned]
since: 0.5.0
---

A class's `__iter__` returns, on every path, a list, tuple, dict, set, number, string or comprehension; `self` when no class of the file's chain defines `__next__`; an instance of a class of this file that defines no `__next__`; or such a class itself (with no metaclass). A method with `yield`, an `async def`, and any returned value the file does not state (a call, an attribute) are not read; one unknown return is enough to say nothing.

**Why:** `for x in obj` calls `iter(obj)`, which raises `TypeError: iter() returned non-iterator` as soon as the loop starts.

```codelaw
native: py/non-iterator-returned
```

## How to fix

1. Return `iter(...)` of the sequence, or
2. make `__iter__` a generator with `yield`, or
3. give the class a `__next__` method when it returns `self`.

## Examples

### Python

#### Don't

```python
class Bag:
    def __init__(self, items):
        self.items = items

    def __iter__(self):
        return []
```

#### Do

```python
class Bag:
    def __init__(self, items):
        self.items = items

    def __iter__(self):
        return iter(self.items)
```
