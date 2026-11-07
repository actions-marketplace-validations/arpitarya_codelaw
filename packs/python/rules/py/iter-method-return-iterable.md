---
id: py/iter-method-return-iterable
title: "__iter__ returns an Iterator"
languages: [python]
files: ["**/*.pyi"]
fix: manual
fix_reason: "the return type becomes `Iterator`, which may need importing"
fix_pending: "suggest; needs add-import (W-162)"
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
message: "Return an iterator type"
```

## How to fix

1. Annotate the return as `Iterator[...]` (`AsyncIterator[...]` for `__aiter__`), importing it from `collections.abc`.

## Examples

### Python

#### Don't

```python path=api.pyi
class Rows:
    def __iter__(self) -> Iterable[int]: ...
```

#### Do

```python path=api.pyi
class Rows:
    def __iter__(self) -> Iterator[int]: ...
```
