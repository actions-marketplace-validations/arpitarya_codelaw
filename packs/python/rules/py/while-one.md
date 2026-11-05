---
id: py/while-one
title: "while True, not while 1"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/UP048]
since: 0.5.0
---

`while 1:` is written `while True:`.

**Why:** `True` says the loop runs until a `break`.

```codelaw
match:
  kind: integer
  regex: '^1$'
  inside: {kind: while_statement, field: condition}
message: "Write `while True:`"
fix: "True"
```

## Examples

### Python

#### Don't

```python
while 1:
    step()
```

#### Do

```python
while True:
    step()
```
