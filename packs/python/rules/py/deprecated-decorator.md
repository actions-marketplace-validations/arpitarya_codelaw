---
id: py/deprecated-decorator
title: "No decorator the standard library has deprecated"
languages: [python]
fix: manual
fix_reason: "each replacement is a different construct — two decorators, an `async def`, or a rewritten loader — so the edit is a restructure"
fix_exception: restructure
stability: preview
ported_from: [pylint/deprecated-decorator]
since: 0.5.0
---

A decorator is one the standard library deprecates: `abc.abstractclassmethod`, `abc.abstractstaticmethod` and `abc.abstractproperty` (since 3.3), `asyncio.coroutine` (deprecated in 3.8, gone in 3.11), `importlib.util.module_for_loader` (3.4) and `typing.no_type_check_decorator` (3.13). The name is read as imported from its module or written with the module's name. Pylint checks only a function's first decorator; this rule checks each of them.

**Why:** A deprecated decorator warns today and is removed in a later Python.

```codelaw
match:
  kind: decorator
  has:
    pattern: $D
    regex: '^((abc\.)?(abstractclassmethod|abstractstaticmethod|abstractproperty)|(asyncio\.)?coroutine|(importlib\.util\.)?module_for_loader|(typing\.)?no_type_check_decorator)$'
where:
  $D: {imported-from: [abc, asyncio, importlib.util, typing]}
message: "`$D` is deprecated"
```

## How to fix

1. `abstractclassmethod`, `abstractstaticmethod`, `abstractproperty`: stack `@classmethod`, `@staticmethod` or `@property` above `@abstractmethod`.
2. `asyncio.coroutine`: make the function `async def` and use `await` in place of `yield from`.
3. `module_for_loader`: implement `exec_module` on the loader instead.
4. `no_type_check_decorator`: drop it, and mark the functions with `@no_type_check`.

## Examples

### Python

#### Don't

```python
import abc


class Base(abc.ABC):
    @abc.abstractproperty
    def name(self): ...
```

#### Do

```python
import abc


class Base(abc.ABC):
    @property
    @abc.abstractmethod
    def name(self): ...
```
