---
id: py/f-string-number-format
title: "An f-string formats a number in binary, octal or hex"
languages: [python]
fix: suggest
fix_reason: "for a negative number `bin(-5)[2:]` gives `b101` and the format gives `-101`; the edit is reviewed"
stability: preview
ported_from: [ruff/FURB116]
since: 0.5.0
---

`bin(x)[2:]`, `oct(x)[2:]` and `hex(x)[2:]` — the digits without the prefix — are written `f"{x:b}"`, `f"{x:o}"` and `f"{x:x}"`.

**Why:** The format spec says what is meant, without building and slicing a second string.

```codelaw
match:
  kind: subscript
  pattern: '$F($X)[2:]'
constraints:
  F: {regex: '^(bin|oct|hex)$'}
  X: {any: [{kind: identifier}, {kind: attribute}, {kind: integer}]}
message: "Write `f\"{$X:…}\"`"
fix: 'f"{$X:$F|map(bin:b, oct:o, hex:x)}"'
```

## Examples

### Python

#### Don't

```python
bits = bin(mask)[2:]
```

#### Do

```python
bits = f"{mask:b}"
```
