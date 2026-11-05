---
id: py/int-on-sliced-str
title: "int(s, 0), not int(s[2:], 16)"
languages: [python]
fix: suggest
fix_reason: "base 0 also accepts the other prefixes and underscores"
stability: preview
ported_from: [ruff/FURB166]
since: 0.5.0
---

`int(s[2:], 16)` (stripping a `0x` prefix) is written `int(s, 0)`.

**Why:** Base 0 reads the prefix itself.

```codelaw
match:
  any:
    - pattern: int($S[2:], 16)
    - pattern: int($S[2:], 8)
    - pattern: int($S[2:], 2)
message: "Write `int($S, 0)`"
fix: "int($S, 0)"
```

## Examples

### Python

#### Don't

```python
n = int(text[2:], 16)
```

#### Do

```python
n = int(text, 0)
```
