---
id: py/signature-differs
title: "An override keeps the defaults of the method it overrides"
languages: [python]
fix: manual
fix_reason: "whether the base's default or the override's requirement is right is the author's"
fix_exception: restructure
stability: preview
ported_from: [pylint/signature-differs]
since: 0.5.0
---

A method overrides one from an ancestor in this file with the same parameters, but one the base makes optional (it has a default) the override requires. A call written for the base (`obj.run(1)`) fails on the subclass. Fewer, more or renamed parameters are `py/arguments-differ` and `py/arguments-renamed`; this is the case where the count agrees and only the defaults do not. `__init__`, `__new__`, `__call__` and the other dunders `py/arguments-differ` skips, static methods, properties, overloads, and methods where either side takes `*args` are not read. Read from the classes of this file: a base the file imports makes the ancestry open, and the rule then says nothing.

**Why:** `Base().run()` works and `Child().run()` raises `TypeError`, though a `Child` is meant to be usable wherever a `Base` is.

```codelaw
native: py/signature-differs
```

## How to fix

1. Give the override the base's default, or
2. remove the default from the base and update its callers.

## Examples

### Python

#### Don't

```python
class Base:
    def run(self, retries=3, delay=None):
        pass


class Child(Base):
    def run(self, retries, delay=None):
        pass
```

#### Do

```python
class Base:
    def run(self, retries=3, delay=None):
        pass


class Child(Base):
    def run(self, retries=3, delay=None):
        pass
```
