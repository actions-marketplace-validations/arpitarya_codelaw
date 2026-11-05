---
id: py/non-pep585-annotation
title: "list[int], not typing.List[int]"
languages: [python]
fix: suggest
fix_reason: "the builtin generic needs Python 3.9; the `typing` import may become unused"
stability: preview
ported_from: [ruff/UP006, pylint/deprecated-typing-alias, pylint/consider-using-alias]
since: 0.5.0
---

Annotations use the builtin generics `list`, `dict`, `set`, `frozenset`, `tuple` and `type` instead of `typing.List`, `Dict`, `Set`, `FrozenSet`, `Tuple` and `Type` (Python 3.9+). The fix covers `List`.

**Why:** The `typing` aliases are deprecated since Python 3.9.

```codelaw
match:
  kind: identifier
  regex: '^List$'
  inside: {kind: generic_type, inside: {kind: type, stopBy: end}}
message: "Write `list`"
fix: "list"
```

## Examples

### Python

#### Don't

```python
def f(xs: List[int]) -> None:
    pass
```

#### Do

```python
def f(xs: list[int]) -> None:
    pass
```
