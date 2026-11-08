---
id: py/arguments-renamed
title: "An override names its parameters like the method it overrides"
languages: [python]
fix: manual
fix_reason: "which name is right, the base's or the override's, is the author's"
fix_exception: restructure
stability: preview
ported_from: [pylint/arguments-renamed]
since: 0.5.0
---

A method overrides one from an ancestor in this file with the same number of positional parameters and a different name for one of them (names that start with an underscore are left alone). Other differences are `py/arguments-differ`; the same exemptions apply. Read from the classes of this file: a base the file imports makes the ancestry open, and the rule then says nothing; a base in another file is not followed (the project graph's class facts would answer it).

**Why:** A caller that passes the argument by keyword (`obj.save(path='x')`) fails on the subclass.

```codelaw
native: py/arguments-renamed
```

## How to fix

1. Rename the override's parameter to the base's name.

## Examples

### Python

#### Don't

```python
class Base:
    def save(self, path):
        pass


class Child(Base):
    def save(self, target):
        pass
```

#### Do

```python
class Base:
    def save(self, path):
        pass


class Child(Base):
    def save(self, path):
        pass
```
