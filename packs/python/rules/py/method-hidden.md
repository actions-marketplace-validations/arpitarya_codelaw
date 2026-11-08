---
id: py/method-hidden
title: "An attribute does not hide a method of its class"
languages: [python]
fix: manual
fix_reason: "which of the two names changes is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/method-hidden]
since: 0.5.0
---

A method `m` of a class and an assignment `self.m = …` in a method of the same class. A method that is a property (or its setter or an overload) is not read.

**Why:** The instance attribute shadows the method, so calling `obj.m()` calls whatever was assigned, or fails.

```codelaw
native: py/method-hidden
```

## How to fix

1. Rename the attribute or the method.

## Examples

### Python

#### Don't

```python
class Door:
    def __init__(self):
        self.open = False

    def open(self):
        return True
```

#### Do

```python
class Door:
    def __init__(self):
        self.is_open = False

    def open(self):
        return True
```
