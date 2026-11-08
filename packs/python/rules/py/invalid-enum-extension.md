---
id: py/invalid-enum-extension
title: "An enum that defines members is not subclassed"
languages: [python]
fix: manual
fix_reason: "whether the members belong in a base or the subclass should not be one is the author's"
fix_exception: restructure
stability: preview
ported_from: [pylint/invalid-enum-extension]
since: 0.5.0
---

A class of this file derives from an enum class of this file that already defines members (`Enum`, `IntEnum`, `StrEnum`, `Flag` and `IntFlag` count; so do enums derived from them). Only an enum without members can be a base. Read from the classes of this file: a base the file imports makes the ancestry open, and the rule then says nothing; a base in another file is not followed (the project graph's class facts would answer it).

**Why:** Python raises `TypeError: cannot extend enumeration`.

```codelaw
native: py/invalid-enum-extension
```

## How to fix

1. Put the shared behaviour in a base enum with no members, or
2. use a plain class or a new enum with all the members.

## Examples

### Python

#### Don't

```python
from enum import Enum


class Color(Enum):
    RED = 1


class Shade(Color):
    DARK = 2
```

#### Do

```python
from enum import Enum


class Color(Enum):
    RED = 1
    DARK = 2
```
