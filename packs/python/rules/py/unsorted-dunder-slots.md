---
id: py/unsorted-dunder-slots
title: "`__slots__` is sorted"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/RUF023]
since: 0.5.0
---

A class's `__slots__`, a list, tuple or set of string literals, lists its names in natural order (`a2` before `a10`, case-sensitive). When a comment sits inside the literal the finding is left for you, since the comment's place would be lost; a dict of slots with docstrings, and a `__slots__` built any other way, is not read.

**Why:** A sorted `__slots__` is easy to scan and to keep in step with the attributes the class sets.

```codelaw
native: py/unsorted-dunder-slots
```

## Examples

### Python

#### Don't

```python
class Point:
    __slots__ = ("y", "x")
```

#### Do

```python
class Point:
    __slots__ = ("x", "y")
```
