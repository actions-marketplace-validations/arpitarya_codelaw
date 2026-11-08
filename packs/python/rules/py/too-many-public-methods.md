---
id: py/too-many-public-methods
title: "A class has at most `max` public methods"
languages: [python]
fix: manual
fix_reason: "splitting the code is a design choice: which parts belong together is the author's"
fix_exception: restructure
stability: preview
options:
  max: { type: "integer", default: 20, description: "the most public methods a class may define" }
ported_from: [ruff/PLR0904, pylint/too-many-public-methods]
since: 0.5.0
---

A class defining more than `max` public methods (20 by default) is reported. As Ruff counts them: a method whose name has no leading underscore, or a dunder (`__len__`); not a property's setter or deleter, not an `@overload` stub. Stub files are not read.

**Why:** A class with many public methods has many reasons to change; splitting it gives each a name.

```codelaw
native: py/too-many-public-methods
```

## How to fix

1. Split the class by responsibility, or move helpers to functions.

## Examples

### Python

#### Don't

```python options={"max":2}
class Shop:
    def buy(self):
        pass

    def sell(self):
        pass

    def ship(self):
        pass
```

#### Do

```python options={"max":2}
class Shop:
    def buy(self):
        pass

    def sell(self):
        pass
```
