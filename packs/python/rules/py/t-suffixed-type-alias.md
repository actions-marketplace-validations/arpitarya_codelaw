---
id: py/t-suffixed-type-alias
title: "A private type alias does not end in T"
languages: [python]
files: ["**/*.pyi"]
fix: manual
fix_reason: "the alias's new name is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/PYI043]
since: 0.5.0
---

A private type alias in a stub is not named with a `T` suffix (`_FooT`), which is the convention for a `TypeVar`.

**Why:** A reader takes `_FooT` for a type variable.

```codelaw
match:
  kind: assignment
  all:
    - has: {field: left, kind: identifier, regex: '^_\w*[a-z0-9]T$'}
    - has: {field: type, regex: '^(typing\.|typing_extensions\.)?TypeAlias$'}
message: "A `T` suffix marks a TypeVar, not an alias"
```

## How to fix

1. Rename the alias without the `T` suffix and update its uses.

## Examples

### Python

#### Don't

```python path=api.pyi
_IntT: TypeAlias = int
```

#### Do

```python path=api.pyi
_Int: TypeAlias = int
```
