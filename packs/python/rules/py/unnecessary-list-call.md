---
id: py/unnecessary-list-call
title: "No list() around a list comprehension"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/C411]
since: 0.5.0
---

`list([x for x in y])` is written `[x for x in y]`.

**Why:** The comprehension is already a list.

```codelaw
match:
  pattern: list($L)
constraints:
  L: {kind: list_comprehension}
message: "The comprehension is already a list"
fix: "$L"
```

## Examples

### Python

#### Don't

```python
squares = list([n * n for n in nums])
```

#### Do

```python
squares = [n * n for n in nums]
```
