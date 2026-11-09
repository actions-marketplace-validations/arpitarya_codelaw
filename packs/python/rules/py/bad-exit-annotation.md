---
id: py/bad-exit-annotation
title: "`__exit__` and `__aexit__` are annotated as typeshed expects"
languages: [python]
fix: suggest
fix_reason: "`object` is a valid annotation for a star-args parameter; the other findings name an annotation the author must choose, so they carry no edit"
stability: preview
ported_from: [ruff/PYI036]
since: 0.5.0
---

The three arguments of `__exit__` (`__aexit__` when async) are annotated `object`, or `type[BaseException] | None`, `BaseException | None` and `types.TracebackType | None`; star-args in them are annotated `object`; a method without star-args has at least three positional arguments besides `self`, every one past the third has a default and so has every keyword-only argument. A pair of `@overload`s with exactly three such arguments each is either `None, None, None` or `type[BaseException], BaseException, TracebackType`. The star-args finding offers the edit to `object`.

**Why:** A context manager's `__exit__` receives the exception triple; a type checker that sees another annotation rejects `with` statements that are fine.

```codelaw
native: py/bad-exit-annotation
```

## Examples

### Python

#### Don't

```python
class Resource:
    def __exit__(self, *args: int) -> None: ...
```

#### Do

```python
class Resource:
    def __exit__(self, *args: object) -> None: ...
```
