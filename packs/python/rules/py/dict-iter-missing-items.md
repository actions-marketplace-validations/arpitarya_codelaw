---
id: py/dict-iter-missing-items
title: "A dict iterated into two names is read through `.items()`"
languages: [python]
fix: suggest
fix_reason: "`.items()` is what unpacking into a key and a value needs, but the loop may have meant to unpack the keys themselves"
stability: preview
ported_from: [ruff/PLE1141]
since: 0.5.0
---

`for k, v in d:` where `d` is a name bound once to a dict (a `{...}` display, a dict comprehension, `dict()`, `defaultdict()`, `OrderedDict()`, or an annotation `dict[...]`/`Dict[...]`). Keys that are all pairs (`{(1, 2): 'a'}`, or `dict[tuple[int, int], str]`) are left alone. The type comes from where this file states it — a literal, a `dict()`/`set()` call, a comprehension or an annotation — for a name bound once in its scope and never reassigned; a value from a call into other code is unknown and never reported.

**Why:** Iterating a dict gives its keys, so unpacking each key into two names fails, or splits a string key into characters.

```codelaw
native: py/dict-iter-missing-items
```

## How to fix

1. Iterate `d.items()`.

## Examples

### Python

#### Don't

```python
prices = {'tea': 3, 'coffee': 4}
for name, price in prices:
    print(name, price)
```

#### Fixed

```python
prices = {'tea': 3, 'coffee': 4}
for name, price in prices.items():
    print(name, price)
```

#### Do

```python
prices = {'tea': 3, 'coffee': 4}
for name, price in prices.items():
    print(name, price)
```
