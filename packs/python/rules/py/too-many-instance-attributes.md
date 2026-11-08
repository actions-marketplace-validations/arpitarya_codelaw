---
id: py/too-many-instance-attributes
title: "A class sets at most `max` instance attributes"
languages: [python]
fix: manual
fix_reason: "splitting the code is a design choice: which parts belong together is the author's"
fix_exception: restructure
stability: preview
options:
  max: { type: "integer", default: 7, description: "the most instance attributes a class's methods may set" }
ported_from: [pylint/too-many-instance-attributes]
since: 0.5.0
---

A class whose methods assign more than `max` distinct attributes on their first parameter (`self.x = …`; 7 by default, as Pylint) is reported on its name. Static and class methods are not read, nor attributes set from outside the class or by a base class in another file (Pylint infers those).

**Why:** Many attributes mean the object holds several things at once; grouping them in smaller objects names each.

```codelaw
native: py/too-many-instance-attributes
```

## How to fix

1. Group related attributes in their own class, or a dataclass.

## Examples

### Python

#### Don't

```python options={"max":2}
class Job:
    def __init__(self):
        self.a = 1
        self.b = 2
        self.c = 3
```

#### Do

```python options={"max":2}
class Job:
    def __init__(self):
        self.a = 1
        self.b = 2
```
