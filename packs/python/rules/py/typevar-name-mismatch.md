---
id: py/typevar-name-mismatch
title: "A TypeVar's name matches its variable"
languages: [python]
fix: suggest
fix_reason: "the TypeVar's name string changes, which shows in error messages and reprs"
stability: preview
ported_from: [ruff/PLC0132, pylint/typevar-name-mismatch]
since: 0.5.0
---

`T = TypeVar("U")` is written `T = TypeVar("T")`.

**Why:** Type checkers require the name to match; a mismatch is reported and confuses readers.

```codelaw
match:
  kind: assignment
  all:
  - has: {field: left, kind: identifier, pattern: $N}
  - has:
      field: right
      kind: call
      all:
      - has: {field: function, regex: '^(typing\.)?(TypeVar|ParamSpec|TypeVarTuple)$'}
      - has:
          field: arguments
          has:
            kind: string
            nthChild: 1
            pattern: $S
            has:
              kind: string_content
              not: {pattern: $N}
message: Name the TypeVar `"$N"`
fix:
- replace: {target: $S, with: '"$N"'}
```

## Examples

### Python

#### Don't

```python
T = TypeVar("U")
```

#### Do

```python
T = TypeVar("T")
```
