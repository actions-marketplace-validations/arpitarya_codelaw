---
id: py/bad-version-info-order
title: "A version branch in a stub puts the newer version first"
languages: [python]
files: ["**/*.pyi"]
fix: manual
fix_reason: "the condition flips and the branches swap"
fix_exception: restructure
stability: preview
ported_from: [ruff/PYI066]
since: 0.5.0
---

An `if sys.version_info < (...)` that has an `else` is written `if sys.version_info >= (...)`, its branches swapped.

**Why:** Stubs read newest first; the `>=` form also matches what the other checks in typeshed do.

```codelaw
match:
  kind: if_statement
  all:
    - has: {field: condition, regex: '^sys\.version_info\s*<\s*'}
    - has: {kind: else_clause}
message: "Test `>=` and put the newer branch first"
```

## How to fix

1. Write `if sys.version_info >= (...)` and swap the two branches.

## Examples

### Python

#### Don't

```python path=api.pyi
if sys.version_info < (3, 10):
    def f() -> int: ...
else:
    def f() -> str: ...
```

#### Do

```python path=api.pyi
if sys.version_info >= (3, 10):
    def f() -> str: ...
else:
    def f() -> int: ...
```
