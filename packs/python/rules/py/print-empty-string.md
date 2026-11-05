---
id: py/print-empty-string
title: "print(), not print('')"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/FURB105]
since: 0.5.0
---

`print("")` is written `print()`.

**Why:** An empty string adds nothing to the newline `print()` already writes.

```codelaw
match:
  kind: call
  all:
  - has: {field: function, regex: ^print$}
  - has: {field: arguments, regex: ^\(\s*(""|'')\s*\)$}
message: Write `print()`
fix: print()
```

## Examples

### Python

#### Don't

```python
print("")
```

#### Do

```python
print()
```
