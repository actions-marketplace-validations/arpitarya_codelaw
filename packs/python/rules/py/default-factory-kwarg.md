---
id: py/default-factory-kwarg
title: "defaultdict's factory is positional"
languages: [python]
fix: suggest
fix_reason: "the keyword becomes the first argument; a call that also passes a mapping is left to review"
stability: preview
ported_from: [ruff/RUF026]
since: 0.5.0
---

`defaultdict(default_factory=list)` passes the factory positionally: `defaultdict(list)`.

**Why:** `defaultdict` takes its factory only by position; the keyword becomes a key in the dict and the factory stays `None`.

```codelaw
match:
  kind: call
  all:
    - has: {field: function, regex: '^(collections\.)?defaultdict$'}
    - has:
        field: arguments
        regex: '^\(\s*default_factory\s*=[^,]*,?\s*\)$'
        has: {kind: keyword_argument, has: {field: value, pattern: $F}}
message: "`default_factory=` becomes a key; pass the factory positionally"
fix: "defaultdict($F)"
```

## Examples

### Python

#### Don't

```python
counts = defaultdict(default_factory=int)
```

#### Do

```python
counts = defaultdict(int)
```
