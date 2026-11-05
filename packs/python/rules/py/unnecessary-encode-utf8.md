---
id: py/unnecessary-encode-utf8
title: "No .encode('utf-8') on a literal"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/UP012]
since: 0.5.0
---

`"text".encode("utf-8")` on an ASCII string literal is written `b"text"`; `.encode("utf-8")` is written `.encode()`.

**Why:** UTF-8 is the default encoding.

```codelaw
match:
  kind: call
  all:
  - has: {field: function, kind: attribute, regex: \.encode$, pattern: $M}
  - has: {field: arguments, regex: '^\(\s*["''](utf-?8|UTF-?8)["'']\s*\)$'}
message: UTF-8 is the default; write `.encode()`
fix: $M()
```

## Examples

### Python

#### Don't

```python
data = text.encode("utf-8")
```

#### Do

```python
data = text.encode()
```
