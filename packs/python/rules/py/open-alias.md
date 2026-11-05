---
id: py/open-alias
title: "open(), not io.open()"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/UP020]
since: 0.5.0
---

`io.open` is written `open`.

**Why:** In Python 3 they are the same function.

```codelaw
match:
  kind: attribute
  regex: '^io\.open$'
  inside: {kind: call, field: function}
message: "Write `open`"
fix: "open"
```

## Examples

### Python

#### Don't

```python
f = io.open(path)
```

#### Do

```python
f = open(path)
```
