---
id: py/invalid-overridden-method
title: "An override keeps the kind of the method it overrides"
languages: [python]
fix: manual
fix_reason: "whether the base or the override is the one to change is the author's"
fix_exception: restructure
stability: preview
ported_from: [pylint/invalid-overridden-method]
since: 0.5.0
---

A method overrides one from an ancestor in this file and differs in kind: a `property` overridden by a method or the other way round, or an `async def` overridden by a plain `def` or the other way round. Dunder methods are not read. Read from the classes of this file: a base the file imports makes the ancestry open, and the rule then says nothing; a base in another file is not followed (the project graph's class facts would answer it).

**Why:** Callers written against the base `await` the result or read it without a call; the override breaks both.

```codelaw
native: py/invalid-overridden-method
```

## How to fix

1. Make the override the same kind as the base, or
2. change the base if the override is the intended shape.

## Examples

### Python

#### Don't

```python
class Base:
    async def fetch(self): ...


class Child(Base):
    def fetch(self): ...
```

#### Do

```python
class Base:
    async def fetch(self): ...


class Child(Base):
    async def fetch(self): ...
```
