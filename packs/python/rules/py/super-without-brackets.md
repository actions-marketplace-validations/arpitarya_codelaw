---
id: py/super-without-brackets
title: "super(), not super"
languages: [python]
fix: auto
stability: preview
ported_from: [ruff/PLW0245]
since: 0.5.0
---

`super.method()` is written `super().method()`.

**Why:** `super` without the call is the builtin type, and the attribute lookup fails.

```codelaw
match:
  kind: attribute
  has: {field: object, kind: identifier, regex: '^super$'}
  pattern: super.$A
message: "Call `super()` first"
fix: "super().$A"
```

## Examples

### Python

#### Don't

```python
class A(B):
    def __init__(self):
        super.__init__()
```

#### Do

```python
class A(B):
    def __init__(self):
        super().__init__()
```
