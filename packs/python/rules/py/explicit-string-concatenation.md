---
id: py/explicit-string-concatenation
title: "No + between two string literals"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/ISC003]
since: 0.5.0
---

Two plain string literals joined with `+` are written as one implicit concatenation, `"a" "b"`.

**Why:** The parser joins adjacent literals for free; `+` is an operation at runtime.

```codelaw
match:
  kind: binary_operator
  pattern: $A + $B
  not: {inside: {kind: binary_operator}}
constraints:
  A: {kind: string, not: {regex: '^[rRbBuU]*[fFbB]'}}
  B: {kind: string, not: {regex: '^[rRbBuU]*[fFbB]'}}
message: "Drop the `+`"
fix: "$A $B"
```

## Examples

### Python

#### Don't

```python
msg = "a" + "b"
```

#### Do

```python
msg = "a" "b"
```
