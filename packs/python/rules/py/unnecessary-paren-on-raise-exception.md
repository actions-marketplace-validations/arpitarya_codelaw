---
id: py/unnecessary-paren-on-raise-exception
title: "raise ValueError, not raise ValueError()"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/RSE102]
since: 0.5.0
---

An exception raised with no arguments is written without empty parentheses.

**Why:** `raise` instantiates the class itself.

```codelaw
match:
  kind: call
  all:
  - has: {field: arguments, regex: ^\(\s*\)$}
  - has: {field: function, pattern: $E, regex: '^[A-Z]\w*(Error|Exception|Exit|Interrupt|Warning|Iteration)$|^[A-Z][A-Za-z]*Error$'}
  - inside: {kind: raise_statement}
message: Drop the empty parentheses
fix: $E
```

## Examples

### Python

#### Don't

```python
raise ValueError()
```

#### Do

```python
raise ValueError
```
