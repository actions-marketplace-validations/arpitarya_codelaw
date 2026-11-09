---
id: py/access-member-before-definition
title: "An attribute is assigned before `self` reads it"
languages: [python]
fix: manual
fix_reason: "where the attribute should first be assigned, or what the read should see before then, is the author's"
fix_exception: restructure
stability: preview
ported_from: [pylint/access-member-before-definition]
since: 0.5.0
---

A method reads `self.x` on a line before the line that first assigns it in the same method, and no other method of the class or of an in-file ancestor assigns `x`, and the class has no class-level `x`. Also in `__init__`: a `self.method()` call before the line that assigns an attribute `method` reads without assigning it first. A read inside a `try` whose handler catches `AttributeError` (or everything), a read in a nested function or lambda, a bare `self.x: int` annotation, a class whose name ends in `Mixin`, and a class with a base the file imports are not read.

**Why:** The first read raises `AttributeError` (or reads whatever a subclass assigned), so the code only works by accident.

```codelaw
native: py/access-member-before-definition
```

## How to fix

1. Assign the attribute before the first read, or
2. give it a class-level default.

## Examples

### Python

#### Don't

```python
class Counter:
    def __init__(self):
        total = self.count
        self.count = 0
        self.total = total
```

#### Do

```python
class Counter:
    def __init__(self):
        self.count = 0
        total = self.count
        self.total = total
```
