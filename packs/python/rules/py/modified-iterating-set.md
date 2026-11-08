---
id: py/modified-iterating-set
title: "A set is not changed while a loop iterates it"
languages: [python]
fix: suggest
fix_reason: "iterating a copy changes what the loop sees after the first change, which is the point but is the author's to confirm"
stability: preview
ported_from: [ruff/PLE4703]
since: 0.5.0
---

A `for` loop iterates a name bound once to a set (a `{1, 2}` display, a set comprehension, `set()`, or an annotation `set[...]`/`Set[...]`) and its body calls `add`, `clear`, `discard`, `pop` or `remove` on that same name. The type comes from where this file states it — a literal, a `dict()`/`set()` call, a comprehension or an annotation — for a name bound once in its scope and never reassigned; a value from a call into other code is unknown and never reported.

**Why:** Python raises `RuntimeError: Set changed size during iteration`, or skips elements.

```codelaw
native: py/modified-iterating-set
```

## How to fix

1. Iterate a copy: `for x in s.copy():`.

## Examples

### Python

#### Don't

```python
seen = {1, 2, 3}
for n in seen:
    seen.discard(n)
```

#### Fixed

```python
seen = {1, 2, 3}
for n in seen.copy():
    seen.discard(n)
```

#### Do

```python
seen = {1, 2, 3}
for n in seen.copy():
    seen.discard(n)
```
