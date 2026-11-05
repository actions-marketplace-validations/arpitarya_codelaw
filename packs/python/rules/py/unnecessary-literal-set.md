---
id: py/unnecessary-literal-set
title: "A set literal, not set([...])"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/C405]
since: 0.5.0
---

`set([1, 2])` and `set((1, 2))` are written `{1, 2}`.

**Why:** The literal builds the set directly.

```codelaw
match:
  any:
    - pattern: set([$A, $$$B])
    - pattern: set(($A, $$$B))
message: "Write a set literal"
fix: "{$A, $$$B}"
```

## Examples

### Python

#### Don't

```python
tags = set(["a", "b"])
```

#### Do

```python
tags = {"a", "b"}
```
