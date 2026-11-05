---
id: py/subclass-builtin
title: "UserDict, not a dict subclass"
languages: [python]
fix: suggest
fix_reason: "`collections.UserDict` must be imported, and its `data` attribute replaces direct storage"
stability: preview
ported_from: [ruff/FURB189]
since: 0.5.0
---

A class that subclasses `dict` is written as a subclass of `collections.UserDict` (and `list`, `str` of `UserList`, `UserString`). The fix covers `dict`.

**Why:** The builtins' methods bypass overridden ones (`update` ignores your `__setitem__`); the `User*` classes don't.

```codelaw
match:
  kind: identifier
  regex: '^dict$'
  inside: {kind: argument_list, regex: '^\(\s*dict\s*\)$', inside: {kind: class_definition}}
message: "Subclass `collections.UserDict` instead"
fix: "collections.UserDict"
```

## Examples

### Python

#### Don't

```python
class Registry(dict):
    pass
```

#### Do

```python
class Registry(collections.UserDict):
    pass
```
