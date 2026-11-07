---
id: py/snake-case-type-alias
title: "A stub's type aliases are CamelCase"
languages: [python]
files: ["**/*.pyi"]
fix: manual
fix_reason: "every use of the alias changes with its name"
fix_exception: restructure
stability: preview
ported_from: [ruff/PYI042]
since: 0.5.0
---

A type alias declared with `: TypeAlias` in a stub is named in CamelCase: `UserId`, not `user_id`.

**Why:** Type aliases are types, and types are named in CamelCase.

```codelaw
match:
  kind: assignment
  all:
    - has: {field: left, kind: identifier, regex: '^_*[a-z]+_[a-z0-9_]*$'}
    - has: {field: type, regex: '^(typing\.|typing_extensions\.)?TypeAlias$'}
message: "Name the type alias in CamelCase"
```

## How to fix

1. Rename the alias in CamelCase and update its uses.

## Examples

### Python

#### Don't

```python path=api.pyi
user_id: TypeAlias = int
```

#### Do

```python path=api.pyi
UserId: TypeAlias = int
```
