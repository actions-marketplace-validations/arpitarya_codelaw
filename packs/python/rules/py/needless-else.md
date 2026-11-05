---
id: py/needless-else
title: "No empty else"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/RUF047]
since: 0.5.0
---

An `else:` whose body is only `pass` or `...` is removed.

**Why:** It says nothing and makes the reader look for a missing branch.

```codelaw
match:
  kind: else_clause
  has: {field: body, regex: '^(pass|\.\.\.)$'}
  not: {inside: {kind: try_statement}}
message: "This `else` does nothing; remove it"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
if ready:
    go()
else:
    pass
```

#### Do

```python
if ready:
    go()
```
