---
id: py/overridden-final-method
title: "A method decorated `@final` is not overridden"
languages: [python]
fix: manual
fix_reason: "whether to drop `@final` or the override is the author's"
fix_exception: restructure
stability: preview
ported_from: [pylint/overridden-final-method]
since: 0.5.0
---

A class overrides a method that an ancestor in this file declares `@final`. Read from the classes of this file: a base the file imports makes the ancestry open, and the rule then says nothing; a base in another file is not followed (the project graph's class facts would answer it).

**Why:** `final` says no override is allowed; the base class relies on its own method running as written.

```codelaw
native: py/overridden-final-method
```

## How to fix

1. Rename the method, or
2. remove `@final` from the base if overriding is meant to be allowed.

## Examples

### Python

#### Don't

```python
from typing import final


class Base:
    @final
    def run(self): ...


class Child(Base):
    def run(self): ...
```

#### Do

```python
from typing import final


class Base:
    @final
    def run(self): ...


class Child(Base):
    def start(self): ...
```
