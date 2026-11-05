---
id: py/access-annotations-from-class-dict
title: "annotationlib, not __dict__['__annotations__']"
languages: [python]
fix: suggest
fix_reason: "`inspect.get_annotations` evaluates lazily-defined annotations the dict lookup did not"
stability: preview
ported_from: [ruff/RUF063]
since: 0.5.0
---

A class's annotations are read with `inspect.get_annotations(cls)`, not `cls.__dict__["__annotations__"]`.

**Why:** From Python 3.14 annotations are evaluated lazily and are not in `__dict__`.

```codelaw
match:
  any:
    - pattern: $C.__dict__["__annotations__"]
    - pattern: $C.__dict__.get("__annotations__")
message: "Use `inspect.get_annotations($C)`"
fix: "inspect.get_annotations($C)"
```

## Examples

### Python

#### Don't

```python
hints = Model.__dict__["__annotations__"]
```

#### Do

```python
hints = inspect.get_annotations(Model)
```
