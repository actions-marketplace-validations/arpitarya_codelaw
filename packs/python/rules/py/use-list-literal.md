---
id: py/use-list-literal
title: "[], not list()"
languages: [python]
fix: auto
stability: preview
ported_from: [pylint/use-list-literal]
since: 0.5.0
---

An empty list is written `[]`, not `list()`.

**Why:** The literal is faster and reads as the value.

```codelaw
match:
  pattern: list()
message: "Write `[]`"
fix: "[]"
```

## Examples

### Python

#### Don't

```python
items = list()
```

#### Do

```python
items = []
```
