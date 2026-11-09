---
id: py/unused-private-protocol
title: "A private Protocol is used"
languages: [python]
fix: suggest
fix_reason: "deleting the declaration can break an importer of the private name in another file"
stability: preview
ported_from: [ruff/PYI046]
since: 0.5.0
---

A class derived from `Protocol` whose name starts with one underscore is used in the file. The edit deletes the class.

**Why:** A private protocol nothing refers to is dead code.

```codelaw
native: py/unused-private-protocol
```

## Examples

### Python

#### Don't

```python
from typing import Protocol


class _Reader(Protocol):
    def read(self) -> bytes: ...
```

#### Do

```python
from typing import Protocol


class _Reader(Protocol):
    def read(self) -> bytes: ...


def load(source: _Reader) -> bytes:
    return source.read()
```

#### Fixed

```python
from typing import Protocol
```
