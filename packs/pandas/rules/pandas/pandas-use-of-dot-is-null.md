---
id: pandas/pandas-use-of-dot-is-null
title: ".isna, not .isnull"
languages: [python]
fix: suggest
fix_reason: "the receiver may not be a pandas object; confirm before renaming"
stability: preview
ported_from: [ruff/PD003]
since: 0.5.0
---

`.isna()` is used instead of its alias `.isnull()`.

**Why:** pandas documents `isna`; one name for one operation reads better.

```codelaw
match:
  pattern: $X.isnull($$$A)
  inside: { kind: module, stopBy: end, has: { any: [{kind: import_statement}, {kind: import_from_statement}], regex: '^(import|from)\s+pandas\b' } }
message: "Use `.isna()` instead of `.isnull()`"
fix: "$X.isna($$$A)"
```

## Examples

### Python

#### Don't

```python
import pandas as pd
missing = df.isnull()
```

#### Do

```python
import pandas as pd
missing = df.isna()
```
