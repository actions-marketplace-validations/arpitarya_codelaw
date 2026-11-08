---
id: py/super-init-not-called
title: "`__init__` calls its base class's `__init__`"
languages: [python]
fix: manual
fix_reason: "the arguments the base's `__init__` takes are the author's"
fix_exception: value
stability: preview
ported_from: [pylint/super-init-not-called]
since: 0.5.0
---

A class defines `__init__` and has an ancestor in this file that defines one too, but its `__init__` has no `super(...)` call and no `Base.__init__(...)` call. Read from the classes of this file: a base the file imports makes the ancestry open, and the rule then says nothing; a base in another file is not followed (the project graph's class facts would answer it).

**Why:** The base's attributes are never set up, so methods it provides fail later with `AttributeError`.

```codelaw
native: py/super-init-not-called
```

## How to fix

1. Call `super().__init__(...)` with the base's arguments, at the start of `__init__`.

## Examples

### Python

#### Don't

```python
class Base:
    def __init__(self):
        self.ready = True


class Child(Base):
    def __init__(self):
        self.name = 'child'
```

#### Do

```python
class Base:
    def __init__(self):
        self.ready = True


class Child(Base):
    def __init__(self):
        super().__init__()
        self.name = 'child'
```
