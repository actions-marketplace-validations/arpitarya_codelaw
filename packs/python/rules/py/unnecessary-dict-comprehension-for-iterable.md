---
id: py/unnecessary-dict-comprehension-for-iterable
title: "dict.fromkeys, not {k: v for k in keys}"
languages: [python]
fix: suggest
fix_reason: "with a mutable value every key would share it; the rule only fires on constants"
stability: preview
ported_from: [ruff/C420]
since: 0.5.0
---

`{k: None for k in keys}` (a constant value) is written `dict.fromkeys(keys, None)`.

**Why:** `fromkeys` says every key gets the same value.

```codelaw
match:
  kind: dictionary_comprehension
  pattern: "{$K: $V for $K in $Y}"
  not: {has: {any: [{kind: if_clause}, {kind: for_in_clause, nthChild: 3}]}}
constraints:
  K: {kind: identifier}
  V: {any: [{kind: none}, {kind: integer}, {kind: string}, {kind: 'true'}, {kind: 'false'}, {kind: float}]}
message: "Write `dict.fromkeys($Y, $V)`"
fix: "dict.fromkeys($Y, $V)"
```

## Examples

### Python

#### Don't

```python
seen = {name: False for name in names}
```

#### Do

```python
seen = dict.fromkeys(names, False)
```
