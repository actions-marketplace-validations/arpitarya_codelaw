---
id: py/duplicate-class-field-definition
title: "No class attribute assigned twice"
languages: [python]
fix: suggest
fix_reason: "the earlier assignment is removed; review a use between the two"
stability: preview
ported_from: [ruff/PIE794]
since: 0.5.0
---

A name is assigned once in a class body.

**Why:** The later assignment wins; the earlier one is dead and usually a copy-paste slip.

```codelaw
match:
  kind: expression_statement
  has: {kind: assignment, has: {field: left, kind: identifier, pattern: $N}}
  precedes: {stopBy: end, kind: expression_statement, has: {kind: assignment, has: {field: left, kind: identifier, pattern: $N}}}
  inside: {kind: block, inside: {kind: class_definition}}
message: "`$N` is assigned again below; this value is never used"
fix: [remove]
```

## Examples

### Python

#### Don't

```python
class Config:
    debug = False
    port = 80
    debug = True
```

#### Do

```python
class Config:
    port = 80
    debug = True
```
