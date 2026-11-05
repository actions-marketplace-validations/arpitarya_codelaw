---
id: pandas/pandas-use-of-dot-pivot-or-unstack
title: "pivot_table, not pivot or unstack"
languages: [python]
fix: manual
fix_reason: "`pivot_table` needs an aggregation function the author chooses"
fix_exception: value
stability: preview
ported_from: [ruff/PD010]
since: 0.5.0
---

`.pivot_table()` is used instead of `.pivot()` or `.unstack()`.

**Why:** `pivot_table` handles duplicate entries with an explicit aggregation instead of raising.

```codelaw
match:
  any:
    - pattern: $X.pivot($$$A)
    - pattern: $X.unstack($$$A)
  inside: { kind: module, stopBy: end, has: { any: [{kind: import_statement}, {kind: import_from_statement}], regex: '^(import|from)\s+pandas\b' } }
message: "Use `.pivot_table()` instead of `.pivot()` or `.unstack()`"
```

## How to fix

1. Rewrite as `pivot_table(index=…, columns=…, values=…, aggfunc=…)`.

## Examples

### Python

#### Don't

```python
import pandas as pd
wide = df.pivot(index='d', columns='k', values='v')
```

#### Do

```python
import pandas as pd
wide = df.pivot_table(index='d', columns='k', values='v', aggfunc='sum')
```
