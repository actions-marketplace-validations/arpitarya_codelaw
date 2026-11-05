---
id: py/regex-flag-alias
title: "re.IGNORECASE, not re.I"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/FURB167]
since: 0.5.0
---

`re.I`, `re.M`, `re.S`, `re.X`, `re.A`, `re.L` and `re.U` are written by their full names. The fix covers `re.I`.

**Why:** The full name says what the flag does.

```codelaw
match:
  kind: attribute
  regex: '^re\.I$'
message: "Write `re.IGNORECASE`"
fix: "re.IGNORECASE"
```

## Examples

### Python

#### Don't

```python
pat = re.compile("a", re.I)
```

#### Do

```python
pat = re.compile("a", re.IGNORECASE)
```
