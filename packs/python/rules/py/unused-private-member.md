---
id: py/unused-private-member
title: "A private class member is read inside its class"
languages: [python]
fix: manual
fix_reason: "whether the member is dead code or its use is missing is the author's"
fix_exception: restructure
stability: preview
ported_from: [pylint/unused-private-member]
since: 0.5.0
---

A class defines a name-mangled member — a method, a class variable, or an attribute assigned as `self.__x`, `cls.__x`, `Class.__x` or `type(self).__x` — and nothing in the class reads it: no `self.__x`, `cls.__x`, `Class.__x`, `type(self).__x` or `self.__class__.__x`, and no bare name for a function passed along in the class body (`property(__get)`) or called inside its own method. Reading it on another object (`other.__x`, `self.child.__x`) is not a use. Each defining line is reported.

**Why:** A private member nothing reads is dead code, and its name cannot be reached from a subclass or from outside the class.

```codelaw
native: py/unused-private-member
```

## How to fix

1. Remove the member, or
2. add the use that was meant, or
3. drop the leading double underscore if other code is meant to read it.

## Examples

### Python

#### Don't

```python
class Account:
    def __init__(self):
        self.__balance = 0

    def __audit(self):
        pass
```

#### Do

```python
class Account:
    def __init__(self):
        self.__balance = 0

    def balance(self):
        return self.__balance
```
