---
id: pandas/pandas-use-of-dot-ix
title: "No .ix"
languages: [python]
fix: manual
fix_reason: "`.loc` (labels) or `.iloc` (positions) depends on what the index holds"
stability: preview
ported_from: [ruff/PD007]
since: 0.5.0
---

`.ix` is not used; it was removed from pandas.

**Why:** `.ix` guessed between labels and positions and was removed in pandas 1.0.

```codelaw
match:
  kind: attribute
  has: { field: attribute, regex: '^ix$' }
  inside: { kind: module, stopBy: end, has: { any: [{kind: import_statement}, {kind: import_from_statement}], regex: '^(import|from)\s+pandas\b' } }
message: "`.ix` was removed; use `.loc` or `.iloc`"
```

## How to fix

1. Use `.loc` for labels or `.iloc` for positions.

## Examples

### Python

#### Don't

```python
import pandas as pd
row = df.ix[0]
```

#### Do

```python
import pandas as pd
row = df.iloc[0]
```
