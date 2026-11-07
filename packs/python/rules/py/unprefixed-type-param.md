---
id: py/unprefixed-type-param
title: "A stub's TypeVar is private"
languages: [python]
files: ["**/*.pyi"]
fix: manual
fix_reason: "every use of the name in the stub, and any importer, changes with it"
fix_exception: restructure
stability: preview
ported_from: [ruff/PYI001]
since: 0.5.0
---

In a stub, a `TypeVar`, `ParamSpec` or `TypeVarTuple` is assigned to a name starting with `_`: `_T = TypeVar("_T")`.

**Why:** A stub's public names are its API; a type variable is a helper, not something to import.

```codelaw
match:
  kind: assignment
  all:
    - has: {field: left, kind: identifier, regex: '^[^_]'}
    - has: {field: right, kind: call, has: {field: function, regex: '^(typing\.|typing_extensions\.)?(TypeVar|ParamSpec|TypeVarTuple)$'}}
message: "Name the type variable with a leading `_`"
```

## How to fix

1. Rename the type variable with a leading underscore, and its uses.

## Examples

### Python

#### Don't

```python path=api.pyi
T = TypeVar("T")
```

#### Do

```python path=api.pyi
_T = TypeVar("_T")
```
