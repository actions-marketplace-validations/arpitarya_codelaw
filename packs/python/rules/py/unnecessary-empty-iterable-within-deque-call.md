---
id: py/unnecessary-empty-iterable-within-deque-call
title: "deque(), not deque([])"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/RUF037]
since: 0.5.0
---

An empty deque is `deque()`, not `deque([])` or `deque(())`.

**Why:** The empty literal builds and throws away an object to say nothing.

```codelaw
match:
  kind: call
  all:
    - has: {field: function, regex: '^(collections\.)?deque$', pattern: $F}
    - has: {field: arguments, regex: '^\(\s*(\[\s*\]|\(\s*\)|\{\s*\})\s*\)$'}
message: "`$F()` is already empty"
fix: "$F()"
```

## Examples

### Python

#### Don't

```python
queue = deque([])
```

#### Do

```python
queue = deque()
```
