---
id: pandas/pandas-use-of-dot-not-null
title: ".notna, not .notnull"
languages: [python]
fix: manual
fix_reason: "the receiver may not be a pandas object; confirm before renaming"
stability: preview
ported_from: [ruff/PD004]
since: 0.5.0
---

`.notna()` is used instead of its alias `.notnull()`.

**Why:** pandas documents `notna`; one name for one operation reads better.

```codelaw
match:
  pattern: $X.notnull($$$A)
  inside: { kind: module, stopBy: end, has: { any: [{kind: import_statement}, {kind: import_from_statement}], regex: '^(import|from)\s+pandas\b' } }
message: "Use `.notna()` instead of `.notnull()`"
```

## How to fix

1. Rename the call to `.notna()`.

## Examples

### Python

#### Don't

```python
import pandas as pd
ok = df.notnull()
```

#### Do

```python
import pandas as pd
ok = df.notna()
```
