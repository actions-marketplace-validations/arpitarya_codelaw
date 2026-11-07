---
id: py/duplicate-dict-key
title: "A dict literal doesn't repeat a literal key"
languages: [python]
fix: auto
fix_reason: "auto when the repeat has the same value and running it does nothing: it goes. Otherwise the choices are `keep last`, what Python keeps (the first key's place, the last value), the default, and `keep first`; dropping a value with a call can change what runs"
stability: preview
ported_from: [ruff/F601, pylint/duplicate-key]
since: 0.1.0
---

A dict literal names each literal key (`"a"`, `1`, `True`) once.

**Why:** Only the last value is kept, so the earlier one is silently lost.

```codelaw
native: py/duplicate-dict-key
```

## How to fix

1. Decide which value the key should have.
2. Remove the other entry, or give it its own key.

## Examples

### Python

#### Don't

```py
colors = {"red": 1, "red": 2}
```

#### Fixed

```py
colors = {"red": 2}
```

#### Do

```py
colors = {"red": 1, "blue": 2}
```
