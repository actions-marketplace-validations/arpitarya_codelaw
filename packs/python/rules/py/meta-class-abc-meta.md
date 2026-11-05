---
id: py/meta-class-abc-meta
title: "ABC, not metaclass=ABCMeta"
languages: [python]
fix: suggest
fix_reason: "`ABC` must be imported where only `ABCMeta` was"
stability: preview
ported_from: [ruff/FURB180]
since: 0.5.0
---

`class A(metaclass=abc.ABCMeta)` is written `class A(abc.ABC)`.

**Why:** `ABC` is the base class made for this.

```codelaw
match:
  kind: keyword_argument
  regex: '^metaclass\s*=\s*abc\.ABCMeta$'
  inside: {kind: argument_list, inside: {kind: class_definition}}
message: "Inherit from `abc.ABC`"
fix: "abc.ABC"
```

## Examples

### Python

#### Don't

```python
class Base(metaclass=abc.ABCMeta):
    pass
```

#### Do

```python
class Base(abc.ABC):
    pass
```
