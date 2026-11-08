---
id: py/unbalanced-dict-unpacking
title: "Unpacking a dict literal matches its size"
languages: [python]
fix: manual
fix_reason: "whether a target or an entry is missing is the author's"
fix_exception: value
stability: preview
ported_from: [pylint/unbalanced-dict-unpacking]
since: 0.5.0
---

Unpacking a dict literal's keys into a different number of targets — `a, b = {"x": 1}` — is reported, and so is a `for` whose targets do not match each key (`for a, b in {...}` or `.keys()`), each value (`.values()`) or each item's two values (`.items()`) of a dict literal. A key or value's length is its items' count, or a string's characters. A starred target is allowed where the length is at least the targets'.

**Why:** The unpacking raises `ValueError` the first time it runs.

```codelaw
native: py/unbalanced-dict-unpacking
```

## How to fix

1. Unpack `.items()` for key and value, or match the targets to each key.

## Examples

### Python

#### Don't

```python
for key, value in {"a": 1, "b": 2}:
    print(key, value)
```

#### Do

```python
for key, value in {"a": 1, "b": 2}.items():
    print(key, value)
```
