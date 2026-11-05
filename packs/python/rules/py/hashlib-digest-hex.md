---
id: py/hashlib-digest-hex
title: "hexdigest(), not digest().hex()"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/FURB181]
since: 0.5.0
---

`h.digest().hex()` is written `h.hexdigest()`.

**Why:** `hexdigest` is the same in one call.

```codelaw
match:
  pattern: $H.digest().hex()
message: "Write `$H.hexdigest()`"
fix: "$H.hexdigest()"
```

## Examples

### Python

#### Don't

```python
key = hashlib.sha256(data).digest().hex()
```

#### Do

```python
key = hashlib.sha256(data).hexdigest()
```
