---
id: py/unnecessary-call-around-sorted
title: "No list() around sorted()"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/C413]
since: 0.5.0
---

`list(sorted(x))` is written `sorted(x)`.

**Why:** `sorted` already returns a new list.

```codelaw
match:
  pattern: list(sorted($$$A))
message: "`sorted` already returns a list"
fix: "sorted($$$A)"
```

## Examples

### Python

#### Don't

```python
names = list(sorted(users))
```

#### Do

```python
names = sorted(users)
```
