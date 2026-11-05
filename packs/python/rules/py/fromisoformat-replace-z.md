---
id: py/fromisoformat-replace-z
title: "fromisoformat reads Z"
languages: [python]
fix: suggest
fix_reason: "Python 3.11's `fromisoformat` reads `Z` itself"
stability: preview
ported_from: [ruff/FURB162]
since: 0.5.0
---

`datetime.fromisoformat(s.replace("Z", "+00:00"))` is written `datetime.fromisoformat(s)` (Python 3.11+).

**Why:** Since 3.11 `fromisoformat` accepts the `Z` suffix.

```codelaw
match:
  kind: call
  all:
  - has: {field: function, regex: \.fromisoformat$, pattern: $F}
  - has:
      field: arguments
      has: {kind: call, pattern: '$S.replace("Z", "+00:00")'}
message: '`fromisoformat` reads `Z` itself; drop the replace'
fix: $F($S)
```

## Examples

### Python

#### Don't

```python
when = datetime.fromisoformat(text.replace("Z", "+00:00"))
```

#### Do

```python
when = datetime.fromisoformat(text)
```
