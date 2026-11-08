---
id: py/unpacking-non-sequence
title: "Only a sequence is unpacked"
languages: [python]
fix: manual
fix_reason: "the values meant are the author's"
fix_exception: value
stability: preview
ported_from: [pylint/unpacking-non-sequence]
since: 0.5.0
---

A tuple or list target is not assigned a number, `None` or a boolean.

**Why:** Python raises `TypeError: cannot unpack non-iterable`.

```codelaw
match:
  kind: assignment
  all:
    - has: {field: left, any: [{kind: pattern_list}, {kind: tuple_pattern}, {kind: list_pattern}]}
    - has: {field: right, any: [{kind: integer}, {kind: float}, {kind: none}, {kind: 'true'}, {kind: 'false'}]}
message: "This value cannot be unpacked"
```

## How to fix

1. Assign a sequence with one value per name.

## Examples

### Python

#### Don't

```python
a, b = 1
```

#### Do

```python
a, b = 1, 2
```
