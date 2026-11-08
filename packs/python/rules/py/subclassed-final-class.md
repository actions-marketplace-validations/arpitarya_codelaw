---
id: py/subclassed-final-class
title: "A class decorated `@final` is not subclassed"
languages: [python]
fix: manual
fix_reason: "whether to drop `@final` or the subclass is the author's"
fix_exception: restructure
stability: preview
ported_from: [pylint/subclassed-final-class]
since: 0.5.0
---

A class of this file derives from another class of this file that is decorated `@final` (`typing.final`, `typing_extensions.final`). Read from the classes of this file: a base the file imports makes the ancestry open, and the rule then says nothing; a base in another file is not followed (the project graph's class facts would answer it).

**Why:** `final` says no subclass is allowed; type checkers reject it and the class's own design assumed it.

```codelaw
native: py/subclassed-final-class
```

## How to fix

1. Compose the final class instead of subclassing it, or
2. remove `@final` if it was wrong.

## Examples

### Python

#### Don't

```python
from typing import final


@final
class Base: ...


class Child(Base): ...
```

#### Do

```python
from typing import final


@final
class Base: ...


class Child:
    def __init__(self):
        self.base = Base()
```
