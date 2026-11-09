---
id: py/invalid-format-index
title: "A `str.format` field indexes a value that has that index"
languages: [python]
fix: manual
fix_reason: "which index or argument was meant is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/invalid-format-index]
since: 0.5.0
---

`'{0[3]}'.format(x)` or `'{a[key]}'.format(a=x)` where the argument is a literal the file states (or a name bound once to one) and indexing it with the field's key cannot work: an int, float, bool or `None`, a set, a builtin class such as `object`, a string or list or tuple literal shorter than the index, a non-numeric key on a string, list or tuple, or a dict literal with literal keys that lacks the key. Nested keys (`{0[0][1]}`) are followed through nested literals. A call with `*args` or `**kwargs`, a field whose argument is anything else, and a string that is not a plain literal are not read.

**Why:** `'{0[1]}'.format('a')` raises `IndexError` or `TypeError` when the line runs.

```codelaw
native: py/invalid-format-index
```

## How to fix

1. Use an index or key the value has, or
2. pass the value the format string expects.

## Examples

### Python

#### Don't

```python
name = "{0[1]}".format(["a"])
```

#### Do

```python
name = "{0[0]}".format(["a"])
```
