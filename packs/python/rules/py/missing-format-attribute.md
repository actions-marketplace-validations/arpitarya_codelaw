---
id: py/missing-format-attribute
title: "A `str.format` field names an attribute the value has"
languages: [python]
fix: manual
fix_reason: "which attribute or argument was meant is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/missing-format-attribute]
since: 0.5.0
---

`'{0.missing}'.format(x)` or `'{a.missing}'.format(a=x)` where the argument is a literal the file states (or a name bound once to one) of a builtin type (`int`, `float`, `bool`, `str`, `bytes`, `list`, `tuple`, `dict`, `set`) and that type has no attribute of that name. Dunder attributes, attributes after the first, a call with `*args` or `**kwargs`, a field whose argument is anything else, and a string that is not a plain literal are not read. The attribute tables are those of CPython 3.9 to 3.14.

**Why:** `'{0.real}'.format(1)` works; `'{0.missing}'.format(1)` raises `AttributeError` when the line runs.

```codelaw
native: py/missing-format-attribute
```

## How to fix

1. Name an attribute the value has, or
2. pass the object the format string expects.

## Examples

### Python

#### Don't

```python
text = "{0.missing}".format(2)
```

#### Do

```python
text = "{0.real}".format(2)
```
