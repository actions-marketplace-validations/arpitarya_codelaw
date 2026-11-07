---
id: py/duplicate-dict-key-variable
title: "A dict literal doesn't repeat a variable key"
languages: [python]
fix: auto
fix_reason: "auto when the repeat has the same value and running it does nothing: it goes. Otherwise the choices are `keep last`, what Python keeps (the first key's place, the last value), the default, and `keep first`; dropping a value with a call can change what runs"
stability: preview
ported_from: [ruff/F602]
since: 0.1.0
---

A dict literal uses the same variable as a key at most once.

**Why:** Both entries have the same key at run time, so only the last value is kept.

```codelaw
native: py/duplicate-dict-key-variable
```

## How to fix

1. Decide which entry is meant.
2. Remove the other one, or use a different key.

## Examples

### Python

#### Don't

```py
scores = {player: 1, player: 2}
```

#### Fixed

```py
scores = {player: 2}
```

#### Do

```py
scores = {player: 1, rival: 2}
```
