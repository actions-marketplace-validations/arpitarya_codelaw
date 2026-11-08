---
id: py/bad-super-call
title: "`super()` names the class it is called in"
languages: [python]
fix: suggest
fix_reason: "naming the enclosing class changes which method runs (it stops skipping a level), so it is offered, not applied"
stability: preview
ported_from: [pylint/bad-super-call]
since: 0.5.0
---

`super(X, self)` in a method of class `C`, where `X` is another class of this file. The first argument names the class the lookup starts after, which is the class the method is in; naming another class skips or repeats a level. `super(type(self), self)` and `super(self.__class__, self)` are not read.

**Why:** `super(Parent, self).run()` inside `Child` runs the grandparent's `run`.

```codelaw
native: py/bad-super-call
```

## How to fix

1. Name the enclosing class, or call `super()` with no arguments.

## Examples

### Python

#### Don't

```python
class Base:
    def run(self):
        pass


class Child(Base):
    def run(self):
        super(Base, self).run()
```

#### Fixed

```python
class Base:
    def run(self):
        pass


class Child(Base):
    def run(self):
        super(Child, self).run()
```

#### Do

```python
class Base:
    def run(self):
        pass


class Child(Base):
    def run(self):
        super().run()
```
