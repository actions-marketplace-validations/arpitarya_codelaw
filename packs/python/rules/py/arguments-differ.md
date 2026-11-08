---
id: py/arguments-differ
title: "An override takes the arguments of the method it overrides"
languages: [python]
fix: manual
fix_reason: "which signature is right, the base's or the override's, is the author's"
fix_exception: restructure
stability: preview
ported_from: [pylint/arguments-differ]
since: 0.5.0
---

A method overrides one from an ancestor in this file with fewer positional parameters, with more that have no default, or without a keyword-only parameter the base has (unless the override takes `**kwargs`). `__init__`, `__new__`, `__init_subclass__`, `__post_init__`, `__call__` and `__class_getitem__`, static methods, properties, overloads, and methods where either side takes `*args` are not read. Renamed parameters are `py/arguments-renamed`. Read from the classes of this file: a base the file imports makes the ancestry open, and the rule then says nothing; a base in another file is not followed (the project graph's class facts would answer it).

**Why:** A call written for the base (`obj.save(path, mode)`) fails or misbehaves on the subclass.

```codelaw
native: py/arguments-differ
```

## How to fix

1. Give the override the base's parameters (extra ones with defaults), or
2. change the base and every caller.

## Examples

### Python

#### Don't

```python
class Base:
    def save(self, path, mode):
        pass


class Child(Base):
    def save(self, path):
        pass
```

#### Do

```python
class Base:
    def save(self, path, mode):
        pass


class Child(Base):
    def save(self, path, mode, flush=False):
        pass
```
