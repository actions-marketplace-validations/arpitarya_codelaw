---
id: py/unquoted-type-alias
title: "A type alias that names a `TYPE_CHECKING`-only import is quoted"
languages: [python]
fix: suggest
fix_reason: "the whole value is quoted, so a library that reads the alias at run time sees a string"
stability: preview
ported_from: [ruff/TC007]
since: 0.5.0
---

An explicit alias (`X: TypeAlias = …`) whose value names an import made under `if TYPE_CHECKING:` (through a union, a subscript, an attribute or a list; not inside `Literal[…]` or after the first argument of `Annotated`) is quoted as a whole: `X: TypeAlias = "Foo | None"`. A name the code also uses at run time is not reported (`py/runtime-import-in-type-checking-block` reports that one), and a value with a quote or a line break in it is reported without an edit.

**Why:** The value of a `TypeAlias` runs when the module loads, and a name that exists only under `TYPE_CHECKING` raises a `NameError`.

```codelaw
native: py/unquoted-type-alias
```

## Examples

### Python

#### Don't

```python
from typing import TYPE_CHECKING, TypeAlias

if TYPE_CHECKING:
    from foo import Foo

OptFoo: TypeAlias = Foo | None
```

#### Do

```python
from typing import TYPE_CHECKING, TypeAlias

if TYPE_CHECKING:
    from foo import Foo

OptFoo: TypeAlias = "Foo | None"
```
