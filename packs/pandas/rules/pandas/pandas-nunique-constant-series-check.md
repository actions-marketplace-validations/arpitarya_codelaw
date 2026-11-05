---
id: pandas/pandas-nunique-constant-series-check
title: "No nunique() to test a constant series"
languages: [python]
fix: suggest
fix_reason: "the replacement reads the first value and compares it with the rest; the reviewer confirms how an empty series and NaN should count"
stability: preview
ported_from: [ruff/PD101]
since: 0.5.0
---

Whether a series is constant is not tested with `.nunique() == 1`.

**Why:** `nunique` counts every distinct value; comparing the first value with the rest stops early and is much faster.

```codelaw
match:
  any:
    - all:
        - pattern: $S1.nunique() == 1
        - pattern: $W1
    - all:
        - pattern: $S2.nunique() != 1
        - pattern: $W2
    - all:
        - pattern: $S3.nunique() <= 1
        - pattern: $W3
    - all:
        - pattern: $S4.nunique() > 1
        - pattern: $W4
  inside: { kind: module, stopBy: end, has: { any: [{kind: import_statement}, {kind: import_from_statement}], regex: '^(import|from)\s+pandas\b' } }
message: "Use `(s.iloc[0] == s).all()` to test for a constant series"
fix:
  - replace: { target: $W1, with: "($S1.iloc[0] == $S1).all()" }
  - replace: { target: $W2, with: "(not ($S2.iloc[0] == $S2).all())" }
  - replace: { target: $W3, with: "($S3.iloc[0] == $S3).all()" }
  - replace: { target: $W4, with: "(not ($S4.iloc[0] == $S4).all())" }
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
