---
id: py/duplicate-bases
title: "No base class listed twice"
languages: [python]
fix: suggest
fix_reason: "the second listing is removed; it already made the class fail to define"
stability: preview
ported_from: [ruff/PLE0241]
since: 0.5.0
---

A class does not list the same base twice.

**Why:** That raises TypeError when the class is defined.

```codelaw
match:
  kind: identifier
  pattern: $B
  follows: {stopBy: end, kind: identifier, pattern: $B}
  inside: {kind: argument_list, inside: {kind: class_definition}}
message: "`$B` is already a base"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
class A(Base, Base):
    pass
```

#### Do

```python
class A(Base):
    pass
```
