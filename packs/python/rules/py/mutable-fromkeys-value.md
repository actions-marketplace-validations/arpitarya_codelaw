---
id: py/mutable-fromkeys-value
title: "No mutable value in dict.fromkeys"
languages: [python]
fix: suggest
fix_reason: "the comprehension gives each key its own value, which changes behaviour if sharing was relied on"
stability: preview
ported_from: [ruff/RUF024]
since: 0.5.0
---

`dict.fromkeys(keys, [])` is written as a comprehension, so each key gets its own list.

**Why:** `fromkeys` stores the one value under every key; appending to one appends to all.

```codelaw
match:
  pattern: dict.fromkeys($K, $V)
constraints:
  V: {any: [{kind: list}, {kind: dictionary}, {kind: set}]}
message: "Every key shares this one value; use a comprehension"
fix: "{key: $V for key in $K}"
```

## Examples

### Python

#### Don't

```python
groups = dict.fromkeys(names, [])
```

#### Do

```python
groups = {key: [] for key in names}
```
