---
id: py/too-few-public-methods
title: "A class defines at least `min` public methods"
languages: [python]
fix: manual
fix_reason: "whether the class should be a function, a dataclass or gain behaviour is a design choice"
fix_exception: restructure
stability: preview
options:
  max: { type: "integer", default: 2, description: "the fewest public methods a class may define" }
ported_from: [pylint/too-few-public-methods]
since: 0.5.0
---

A class with no base (or only `object`) that defines fewer than `min` public methods (2 by default, as Pylint; names without a leading underscore) is reported on its name. A dataclass or attrs class (`@dataclass`, `@define`, `@frozen`) is data and is not read; nor is a class with a base, whose inherited methods Pylint counts and a single file does not show.

**Why:** A class with one public method is usually a function with extra steps, and a class with none is usually a record better written as a dataclass.

```codelaw
native: py/too-few-public-methods
```

## How to fix

1. Write it as a function, or as a dataclass if it holds data.

## Examples

### Python

#### Don't

```python
class Greeter:
    def greet(self, name):
        return name
```

#### Do

```python
def greet(name):
    return name
```
