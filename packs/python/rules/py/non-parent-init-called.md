---
id: py/non-parent-init-called
title: "`__init__` is called only on a base class"
languages: [python]
fix: manual
fix_reason: "which base was meant, or whether the class should derive from it, is the author's"
fix_exception: restructure
stability: preview
ported_from: [pylint/non-parent-init-called]
since: 0.5.0
---

`Other.__init__(self, ...)` in a class's `__init__`, where `Other` is a class of this file that is not among the class's ancestors. Read from the classes of this file: a base the file imports makes the ancestry open, and the rule then says nothing; a base in another file is not followed (the project graph's class facts would answer it).

**Why:** `Other.__init__` initialises attributes of a class `self` is not an instance of, or runs an `__init__` the class never meant to inherit.

```codelaw
native: py/non-parent-init-called
```

## How to fix

1. Call `__init__` of a real base (`super().__init__(...)`), or
2. add the class to the bases if it is meant to be one.

## Examples

### Python

#### Don't

```python
class A:
    def __init__(self):
        self.a = 1


class B:
    def __init__(self):
        A.__init__(self)
```

#### Do

```python
class A:
    def __init__(self):
        self.a = 1


class B(A):
    def __init__(self):
        A.__init__(self)
```
