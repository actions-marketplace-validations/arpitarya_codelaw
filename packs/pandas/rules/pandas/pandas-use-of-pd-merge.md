---
id: pandas/pandas-use-of-pd-merge
title: "df.merge, not pd.merge"
languages: [python]
fix: suggest
fix_reason: "`a.merge(b)` needs `a` to be a DataFrame, where `pd.merge` also takes a named Series; review the receiver"
stability: preview
ported_from: [ruff/PD015]
since: 0.5.0
---

Frames are joined with the `.merge()` method, not `pd.merge()`.

**Why:** The method form reads left to right and chains with the rest of the pipeline.

```codelaw
match:
  pattern: pd.merge($L, $$$R)
  inside: { kind: module, stopBy: end, has: { any: [{kind: import_statement}, {kind: import_from_statement}], regex: '^(import|from)\s+pandas\b' } }
message: "Use `left.merge(right, …)` instead of `pd.merge`"
fix: "$L.merge($$$R)"
```

## Examples

### Python

#### Don't

```python
import pandas as pd
out = pd.merge(a, b, on='id')
```

#### Do

```python
import pandas as pd
out = a.merge(b, on='id')
```
