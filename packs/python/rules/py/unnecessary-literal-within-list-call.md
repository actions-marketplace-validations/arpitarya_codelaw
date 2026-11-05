---
id: py/unnecessary-literal-within-list-call
title: "A list literal, not list([...])"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/C410]
since: 0.5.0
---

`list([1, 2])` is written `[1, 2]`.

**Why:** The outer call copies a list just built.

```codelaw
match:
  pattern: list([$$$A])
message: "Drop the `list()` call"
fix: "[$$$A]"
```

## Examples

### Python

#### Don't

```python
names = list(["a", "b"])
```

#### Do

```python
names = ["a", "b"]
```
