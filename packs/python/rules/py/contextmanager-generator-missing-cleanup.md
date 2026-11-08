---
id: py/contextmanager-generator-missing-cleanup
title: "A generator that yields inside a context manager's `with` can be cleaned up"
languages: [python]
fix: manual
fix_reason: "whether the context manager gets a `try/finally`, or the generator stops yielding inside the `with`, is the author's"
fix_exception: restructure
stability: preview
ported_from: [pylint/contextmanager-generator-missing-cleanup]
since: 0.5.0
---

A plain generator function (not itself decorated with `@contextmanager`) yields, inside `with cm() as x:`, a value built from `x`, where `cm` is a `@contextmanager` function of this file whose `yield` hands out a value and has code after it that a closed generator would skip: no `try` with `finally`, and no handler for `GeneratorExit`, `BaseException`, `Exception` or a bare `except`. Context managers from other files and classes with `__exit__` are not read.

**Why:** If the consumer stops before the generator ends, Python raises `GeneratorExit` at the `yield`, the context manager's code after its own `yield` never runs, and the resource stays acquired.

```codelaw
native: py/contextmanager-generator-missing-cleanup
```

## How to fix

1. Wrap the context manager's `yield` in `try/finally` so the cleanup always runs, or
2. collect the values inside the `with` and yield after it.

## Examples

### Python

#### Don't

```python
from contextlib import contextmanager


@contextmanager
def opened():
    handle = open_it()
    yield handle
    handle.close()


def lines():
    with opened() as handle:
        yield handle.read()
```

#### Do

```python
from contextlib import contextmanager


@contextmanager
def opened():
    handle = open_it()
    try:
        yield handle
    finally:
        handle.close()


def lines():
    with opened() as handle:
        yield handle.read()
```
