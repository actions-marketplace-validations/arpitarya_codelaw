---
id: py/single-string-used-for-slots
title: "__slots__ is a tuple, not a string"
languages: [python]
fix: suggest
fix_reason: "a single string already names one slot; the tuple makes that explicit and safe to extend"
stability: preview
ported_from: [ruff/PLC0205, pylint/single-string-used-for-slots]
since: 0.5.0
---

`__slots__ = "name"` is written `__slots__ = ("name",)`.

**Why:** A string is easy to mistake for an iterable of names; adding a second name to it breaks.

```codelaw
match:
  kind: assignment
  all:
  - has: {field: left, regex: ^__slots__$}
  - has: {field: right, kind: string, pattern: $S}
  - inside:
      kind: expression_statement
      inside:
        kind: block
        inside: {kind: class_definition}
message: Write `__slots__ = ($S,)`
fix:
- replace: {target: $S, with: '($S,)'}
```

## Examples

### Python

#### Don't

```python
class A:
    __slots__ = "name"
```

#### Do

```python
class A:
    __slots__ = ("name",)
```
