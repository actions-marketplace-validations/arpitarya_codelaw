---
id: py/consider-using-namedtuple-or-dataclass
title: "A table of same-shaped records is a namedtuple or dataclass"
languages: [python]
fix: manual
fix_reason: "the record's name and its fields' types are the author's"
fix_exception: authored
stability: preview
ported_from: [pylint/consider-using-namedtuple-or-dataclass]
since: 0.5.0
---

A dict literal of two or more entries assigned at module level (or annotated `Final` anywhere) whose every value is a dict literal sharing at least one key (string keys that are valid field names, or names and attributes of string constants this file assigns), or whose every value is a list or tuple of the same non-zero length with no dict element. Keys that are not valid identifiers, values of mixed kinds, lists of different lengths and a dict in a function or class that is not `Final` are not read.

**Why:** A dict of dicts `{'a': {'x': 0, 'y': 1}}` repeats its field names in every row and checks none of them; a dataclass or namedtuple names them once and a type checker follows the fields.

```codelaw
native: py/consider-using-namedtuple-or-dataclass
```

## How to fix

1. Define a `NamedTuple` or `@dataclass` with the shared fields, and build the rows with it.

## Examples

### Python

#### Don't

```python
MAPPING = {
    "a": {"key_1": 0, "key_2": 1},
    "b": {"key_1": 0, "key_2": 1},
}
```

#### Do

```python
from typing import NamedTuple


class Row(NamedTuple):
    key_1: int
    key_2: int


MAPPING = {
    "a": Row(0, 1),
    "b": Row(0, 1),
}
```
