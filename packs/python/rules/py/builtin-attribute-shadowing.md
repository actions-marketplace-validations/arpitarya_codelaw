---
id: py/builtin-attribute-shadowing
title: "A class attribute that shadows a builtin is not read in the class body"
languages: [python]
fix: manual
fix_reason: "the new name is the author's"
fix_exception: value
stability: preview
ported_from: [ruff/A003]
since: 0.5.0
---

A method or attribute of a class takes the name of a Python builtin and the class body itself reads that name later (in a decorator, a default value, an annotation or a statement), so the read gets the attribute and not the builtin. Each such read is reported; the attribute alone, only reachable as `self.name`, is not. A method decorated `@override` is left alone, and under `from __future__ import annotations` annotations are not read.

**Why:** `def repeat(self) -> list[int]` after `def list(self)` annotates with the method, not the builtin type.

```codelaw
native: py/builtin-attribute-shadowing
```

## How to fix

1. Rename the attribute or method, or
2. refer to the builtin as `builtins.list` where the class body needs it.

## Examples

### Python

#### Don't

```python
class Box:
    @staticmethod
    def list():
        return []

    @staticmethod
    def repeat(value, times) -> list[int]:
        return [value] * times
```

#### Do

```python
class Box:
    @staticmethod
    def items():
        return []

    @staticmethod
    def repeat(value, times) -> list[int]:
        return [value] * times
```
