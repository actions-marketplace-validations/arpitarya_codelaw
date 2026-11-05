---
id: py/runtime-cast-value
title: "Quote the type in typing.cast"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/TC006]
since: 0.5.0
---

The type passed to `typing.cast` is a string, `cast("int", x)`.

**Why:** `cast` ignores its type at runtime; a string avoids evaluating it (and importing it) at runtime.

```codelaw
match:
  kind: call
  all:
  - has: {field: function, regex: '^(typing\.)?cast$'}
  - has:
      field: arguments
      has:
        nthChild: 1
        pattern: $T
        all:
        - not: {kind: string}
        - any:
          - {kind: identifier}
          - {kind: attribute}
          - {kind: subscript}
          - {kind: binary_operator}
        - not:
            has: {stopBy: end, kind: string}
message: Write the type as a string
fix:
- replace: {target: $T, with: '"$T"'}
```

## Examples

### Python

#### Don't

```python
n = cast(int, value)
```

#### Do

```python
n = cast("int", value)
```
