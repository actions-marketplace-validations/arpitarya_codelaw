---
id: pandas/pandas-use-of-dot-values
title: ".to_numpy, not .values"
languages: [python]
fix: manual
fix_reason: "the receiver may be a dict-like whose `.values` is right; confirm before replacing"
stability: preview
ported_from: [ruff/PD011]
since: 0.5.0
---

`.to_numpy()` is used instead of the `.values` attribute.

**Why:** `.values` returns different types depending on the data (an array or an extension array); `.to_numpy()` always returns an array.

```codelaw
match:
  kind: attribute
  has: { field: attribute, regex: '^values$' }
  not: { inside: { kind: call, field: function } }
  inside: { kind: module, stopBy: end, has: { any: [{kind: import_statement}, {kind: import_from_statement}], regex: '^(import|from)\s+pandas\b' } }
message: "Use `.to_numpy()` instead of `.values`"
```

## How to fix

1. Replace `.values` with `.to_numpy()`.

## Examples

### Python

#### Don't

```python
import pandas as pd
arr = df['a'].values
```

#### Do

```python
import pandas as pd
arr = df['a'].to_numpy()
```
