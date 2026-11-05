---
id: pandas/pandas-use-of-inplace-argument
title: "No inplace=True"
languages: [python]
fix: manual
fix_reason: "the call must become an assignment of its result, which changes the statement"
fix_pending: "suggest"
stability: preview
ported_from: [ruff/PD002]
since: 0.5.0
---

A pandas method is not called with `inplace=True`; its result is assigned instead.

**Why:** `inplace=True` rarely saves memory, returns `None` (a common source of bugs when chained) and is being deprecated across pandas.

```codelaw
match:
  kind: keyword_argument
  regex: '^inplace\s*=\s*True$'
  inside: { kind: module, stopBy: end, has: { any: [{kind: import_statement}, {kind: import_from_statement}], regex: '^(import|from)\s+pandas\b' } }
message: "Assign the result instead of passing `inplace=True`"
```

## How to fix

1. Remove `inplace=True` and assign the result: `df = df.dropna()`.

## Examples

### Python

#### Don't

```python
import pandas as pd
df.dropna(inplace=True)
```

#### Do

```python
import pandas as pd
df = df.dropna()
```
