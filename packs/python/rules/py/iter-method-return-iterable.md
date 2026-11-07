---
id: py/iter-method-return-iterable
title: "__iter__ returns an Iterator"
languages: [python]
files: ["**/*.pyi"]
fix: suggest
fix_reason: "the type becomes the iterator it returns, which is what `iter()` promises; a bare `Iterator` is imported from where the file imports `Iterable`, and a file that imports neither is fixed by hand"
stability: preview
ported_from: [ruff/PYI045]
since: 0.5.0
---

`__iter__` is annotated to return `Iterator[...]`, and `__aiter__` `AsyncIterator[...]`, not `Iterable` or `AsyncIterable`.

**Why:** `iter()` requires an iterator; an iterable return type lets an object through that `for` cannot use.

```codelaw
match:
  kind: function_definition
  all:
    - has: {field: name, regex: '^__(a)?iter__$'}
    - has: {field: return_type, regex: '^((typing|collections\.abc)\.)?(Iterable|AsyncIterable)\b'}
    - any:
        - all: [{ has: { field: return_type, regex: '^((typing|collections\.abc)\.)(Async)?Iterable\b', pattern: $QUAL } }]
        - all: [{ has: { field: return_type, regex: '^Iterable\b', pattern: $T1 } }, { inside: { kind: module, stopBy: end, has: { kind: import_from_statement, regex: '^from\s+typing\s+import\b[^\n]*\bIterable\b' } } }]
        - all: [{ has: { field: return_type, regex: '^Iterable\b', pattern: $A1 } }, { inside: { kind: module, stopBy: end, has: { kind: import_from_statement, regex: '^from\s+collections\.abc\s+import\b[^\n]*\bIterable\b' } } }]
        - all: [{ has: { field: return_type, regex: '^AsyncIterable\b', pattern: $T2 } }, { inside: { kind: module, stopBy: end, has: { kind: import_from_statement, regex: '^from\s+typing\s+import\b[^\n]*\bAsyncIterable\b' } } }]
        - all: [{ has: { field: return_type, regex: '^AsyncIterable\b', pattern: $A2 } }, { inside: { kind: module, stopBy: end, has: { kind: import_from_statement, regex: '^from\s+collections\.abc\s+import\b[^\n]*\bAsyncIterable\b' } } }]
        - regex: ''
message: "Return an iterator type"
fix:
  - edit-text: { target: $QUAL, pattern: '\A((?:typing|collections\.abc)\.(?:Async)?)Iterable', replace: '${1}Iterator' }
  - edit-text: { target: $T1, pattern: '\AIterable', replace: 'Iterator' }
  - edit-text: { target: $A1, pattern: '\AIterable', replace: 'Iterator' }
  - edit-text: { target: $T2, pattern: '\AAsyncIterable', replace: 'AsyncIterator' }
  - edit-text: { target: $A2, pattern: '\AAsyncIterable', replace: 'AsyncIterator' }
  - ensure-import: { target: $T1, module: typing, name: Iterator }
  - ensure-import: { target: $A1, module: collections.abc, name: Iterator }
  - ensure-import: { target: $T2, module: typing, name: AsyncIterator }
  - ensure-import: { target: $A2, module: collections.abc, name: AsyncIterator }
```

## How to fix

1. Annotate the return as `Iterator[...]` (`AsyncIterator[...]` for `__aiter__`), importing it from `collections.abc`.

## Examples

### Python

#### Don't

```python path=api.pyi
from typing import Iterable


class Rows:
    def __iter__(self) -> Iterable[int]: ...
```

#### Do

```python path=api.pyi
from typing import Iterable, Iterator


class Rows:
    def __iter__(self) -> Iterator[int]: ...
```
