---
id: py/global-at-module-level
title: "No global at module level"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PLW0604]
since: 0.5.0
---

A `global` statement does not appear at module level.

**Why:** Every module-level name is already global; the statement does nothing.

```codelaw
match:
  kind: global_statement
  inside: {kind: module}
message: "`global` at module level does nothing"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
global count
count = 0
```

#### Do

```python
count = 0
```
