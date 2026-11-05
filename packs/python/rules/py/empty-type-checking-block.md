---
id: py/empty-type-checking-block
title: "No empty TYPE_CHECKING block"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/TC005]
since: 0.5.0
---

An `if TYPE_CHECKING:` block whose body is only `pass` is removed.

**Why:** It imports nothing.

```codelaw
match:
  kind: if_statement
  all:
  - has: {field: condition, regex: '^(typing\.)?TYPE_CHECKING$'}
  - has: {field: consequence, regex: ^(pass|\.\.\.)$}
  - not:
      has:
        any:
        - {kind: else_clause}
        - {kind: elif_clause}
message: This block is empty; remove it
fix: [remove]
```

## Examples

### Python

#### Don't

```python
if TYPE_CHECKING:
    pass
x = 1
```

#### Do

```python
x = 1
```
