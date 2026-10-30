---
id: pandas/pandas-nunique-constant-series-check
title: "No nunique() to test a constant series"
languages: [python]
fix: manual
fix_reason: "the faster check is written by hand"
stability: preview
ported_from: [ruff/PD101]
since: 0.5.0
---

Whether a series is constant is not tested with `.nunique() == 1`.

**Why:** `nunique` counts every distinct value; comparing the first value with the rest stops early and is much faster.

```codelaw
match:
  any:
    - pattern: $S.nunique() == 1
    - pattern: $S.nunique() != 1
    - pattern: $S.nunique() <= 1
    - pattern: $S.nunique() > 1
  inside: { kind: module, stopBy: end, has: { any: [{kind: import_statement}, {kind: import_from_statement}], regex: '^(import|from)\s+pandas\b' } }
message: "Use `(s.iloc[0] == s).all()` to test for a constant series"
```

## How to fix

1. Write `s.iloc[0] == s` and `.all()` (mind empty series and NaN).

## Examples

### Python

#### Don't

```python
import pandas as pd
flat = s.nunique() == 1
```

#### Do

```python
import pandas as pd
flat = (s.iloc[0] == s).all()
```
